import numpy as np;
import matplotlib.pyplot as plt;
import scipy.fft as fft;
import typing;



def KLDivergence(  P, Q):
    if len(P) != len(Q):
        return None;
    sum =0;

    for i in range(len(P)):
        sum = np.dot(P, np.log2( np.divide(P, Q)));

    return sum;

def JSDivergence( P, Q):
    M = (P+Q)/2;
    JS = 0.5 * ( KLDivergence(P,M) + KLDivergence(Q, M));
    return JS;

class Sequence2param:



    def __init__ (self, filepath: str, generate: bool ):
        """
        Initializes this instance for a given Sequence's data

        @filepath: string containing the location for data files

        @generate: a boolean telling whether to generate secondary data files from `test.txt`,
        the output of the Rust simulation code. If true, generates files containing acceleration, lattice depth, 
        [acceleration index, lattice depth index] and momentum probabilities respectively.

        """    
        if generate == True:

            data = np.loadtxt(filepath + "test.txt");
            datanew  = data[ np.lexsort((data[:,1],data[:,0])) ]
            AVIndex = datanew[:,0:2]

            AList = np.unique(datanew[:,2]);
            VList = np.unique(datanew[:,3]);

            MomProbdata=datanew[:,4:] #may depend on file, please check
            np.savetxt(filepath + "Acceleration.txt",AList);
            np.savetxt(filepath + "LatticeDepth.txt",VList);

            np.savetxt(filepath + "AVIndex.txt",AVIndex)
            np.savetxt(filepath + "MomProb.txt",MomProbdata) 


        self.working_directory = filepath;
        self.AList = np.loadtxt(filepath + "Acceleration.txt");
        self.VList = np.loadtxt(filepath  + "LatticeDepth.txt");

        self.AVListIndex = np.loadtxt(filepath + "AVIndex.txt", dtype = int);
        self.MomProb = np.loadtxt(filepath + "MomProb.txt"); # np array with rows containing momentum probabilities for each [a,V] value pair in AVList
        self.a0 = 0.0;
        self.V0 = 10.0;


            

    def BayesianUpdating(self, totalmeasurements: int, recordstep: int, totalrecords: int, save_measurement: bool, outcome_file: str, calculate_moments: bool):
        """
        @save_measurement saves the generated outcomes so results can be replicated
        """
        PossibleMomentumOutcomes =np.array( [-10+2*i for i in range(0,11)]); # values of momentum in n\hbar k_L
        PossibleOutcomes = range(0,11);
        datamom =np.reshape(self.MomProb, (self.AList.size,self.VList.size,11));

        P_actual=np.array(datamom[int((self.AList.shape[0]-1)/2),int((self.VList.shape[0]-1)/2),:]); # Fix this hardcoded value #FUCK

        P_actual=P_actual/np.sum(P_actual);
        P_simulated = P_actual; #No errors

        Runs=totalmeasurements; # How many simulated data do we want
        if outcome_file == "":
            outcomes = np.random.default_rng().choice(PossibleOutcomes,size=Runs, p = P_simulated);
        else:
            outcomes = np.loadtxt(self.working_directory + outcome_file).astype(np.int64);
        
        if save_measurement == True:
            np.savetxt(self.working_directory+str(len(outcomes))+".txt", outcomes);
        
        unique, frequency = np.unique(outcomes, return_counts = True);

        PaVprior = np.full((self.AList.size, self.VList.size),1)/(self.AList.size*self.VList.size);

        plotPaV=np.array([PaVprior]);
        
        counter =0;
        stdTimetrue=[];
        stdTimeprob=[];
        meanTime=[];
        for m in outcomes:

            PaVprior*=datamom[:,:,m];
            PaVprior/=np.sum(PaVprior);

            counter+=1;
            if counter % recordstep == 0 and counter < totalrecords: #I forgot to add recordstep == 0 before
                plotPaV=np.append(plotPaV,[PaVprior], axis=0); 
        

            if calculate_moments== True:

                PaVprior_acc = np.sum(PaVprior, axis = 1);
                PaVprior_acc/=np.sum(PaVprior_acc);
                AList = self.AList;
                mean = np.dot(AList, PaVprior_acc)#bayesian mean
                truemean= AList[int((AList.shape[0]-1)/2)]# 0.0;#true mean
                sq = np.dot (np.power(AList, 2), PaVprior_acc)
                meanTime.append(mean);
                stdTimeprob.append(np.sqrt( sq- mean**2));# true mean or bayesian mean
                stdTimetrue.append(np.sqrt( sq- truemean**2));# true mean or bayesian mean


        return (plotPaV, meanTime, stdTimeprob, stdTimetrue);


    def accJSD (self):
        momindices=np.where(self.AVListIndex[:,1]==int((self.VList.shape[0]-1)/2))[0];
        momproblist =self.MomProb[momindices];
        indices = self.AVListIndex[momindices];

        accindices = indices[:, 0];
        acc = self.AList[accindices]

        no_of_values = len(accindices);
        JSDivergenceMatrix=np.zeros((no_of_values, no_of_values)) ;



        for i in accindices:
            for j in accindices:
                JSDivergenceMatrix [i][j]= JSDivergence(momproblist[i], momproblist[j])

        return JSDivergenceMatrix;

    def crossJSD(self):
        momindices_a=np.where(self.AVListIndex[:,1]==int((self.VList.shape[0]-1)/2))[0];
        momproblist_a =self.MomProb[momindices_a];
        indices_a = self.AVListIndex[momindices_a];

        momindices_V=np.where(self.AVListIndex[:,0]==int((self.AList.shape[0]-1)/2))[0];
        momproblist_V = self.MomProb[momindices_V];
        indices_V = self.AVListIndex[momindices_V];

        accindices = indices_a[:, 0];
        acc = self.AList[accindices];

        lattindices = indices_V[:, 1];
        latt = self.VList[lattindices]

        no_of_values_a = len(accindices);
        no_of_values_V = len(lattindices);
        JSDivergenceMatrix=np.zeros((no_of_values_a, no_of_values_V)) ;
    
        for i in accindices:
            for j in lattindices:
                JSDivergenceMatrix [i][j]= JSDivergence(momproblist_a[i], momproblist_V[j])
        return JSDivergenceMatrix;


    def BayesianScan(self, totalmeasurements: int, Vstep: int):
        """
        @save_measurement saves the generated outcomes so results can be replicated
        """
        PossibleMomentumOutcomes =np.array( [-10+2*i for i in range(0,11)]); # values of momentum in n\hbar k_L
        PossibleOutcomes = range(0,11);
        datamom =np.reshape(self.MomProb, (self.AList.size,self.VList.size,11));

        MLE_a =[];
        V_values=[];
        for V_index in range(0, self.VList.shape[0], Vstep):
            P_actual=np.array(datamom[int((self.AList.shape[0]-1)/2),V_index,:]); # Fix this hardcoded value #FUCK

            P_actual=P_actual/np.sum(P_actual);
            P_simulated = P_actual; #No errors

            Runs=totalmeasurements; # How many simulated data do we want
            outcomes = np.random.default_rng().choice(PossibleOutcomes,size=Runs, p = P_simulated);
           
            
            unique, frequency = np.unique(outcomes, return_counts = True);

            PaVprior = np.full(self.AList.size,1)/(self.AList.size);


            counter =0;
            for m in outcomes:
                for i in range(self.AList.size):
                    MomentumProbabilities = datamom[ i, int((self.VList.shape[0]-1)/2), :];

                    PaVprior[i] *= (MomentumProbabilities[m])
                    PaVprior/=np.sum(PaVprior)

            
            MLE_val = self.AList[np.argmax( PaVprior )];
            MLE_a.append(MLE_val);
            V_values.append(self.VList[ V_index]);

        return (V_values, MLE_a);
