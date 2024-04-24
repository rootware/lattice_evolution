from Divergences import KLDivergence;
from Divergences import JSDivergence;

from matplotlib import ticker
momindices=np.where(AVListIndex[:,1]==int((VList.shape[0]-1)/2))[0];
momproblist =MomProb[momindices];
indices = AVListIndex[momindices];

accindices = indices[:, 0];
acc = AList[accindices]

no_of_values = len(accindices);
JSDivergenceMatrix=np.zeros((no_of_values, no_of_values)) ;

for i in accindices:
    for j in accindices:
        JSDivergenceMatrix [i][j]= JSDivergence(momproblist[i], momproblist[j])


plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)

plt.imshow(JSDivergenceMatrix, cmap="plasma_r", extent=[acc[0]*accUnit/g, acc[-1]*accUnit/g, acc[-1]*accUnit/g, acc[0] *accUnit/g])
indices_JS = np.where(JSDivergenceMatrix==0)
plt.plot(AList[indices_JS[1]]*accUnit/g, AList[indices_JS[0]]*accUnit/g)
start_index=0
#plt.plot(AList[indices_JS[1]+start_index]*accUnit/g,np.ones((no_of_values))*0.11, color="white");
#plt.plot(AList[indices_JS[1]+start_index]*accUnit/g,np.ones((no_of_values))*-0.11, color="white");

#plt.plot(np.ones((no_of_values))*0.11, AList[indices_JS[1]+start_index]*accUnit/g,color="white");
#plt.plot(np.ones((no_of_values))*-0.11,AList[indices_JS[1]+start_index]*accUnit/g, color="white");
plt.gca().invert_yaxis();
plt.colorbar()
plt.xlabel("Acceleration ($g$)")
plt.ylabel("Acceleration ($g$)")
plt.title("$V_L$ = "+str(VList[int((VList.shape[0]-1)/2)])+"$E_R$")
plt.grid(False)

#plt.savefig("JS_acc_forlattice.pdf", bbox_inches="tight")