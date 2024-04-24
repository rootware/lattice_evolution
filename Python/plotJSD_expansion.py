from matplotlib import ticker
momindices=np.where(AVListIndex[:,1]==int((VList.shape[0]-1)/2))[0];
momproblist =MomProb[momindices];
indices = AVListIndex[momindices];
datamom =np.reshape( MomProb, ( AList.size, VList.size,11));
accindices = indices[:, 0];
acc = AList[accindices]

no_of_values = len(accindices);
JSDivergenceMatrix=np.zeros((len(AList), len(VList))) ;

for i in range(len(AVListIndex)):
    JSDivergenceMatrix [AVListIndex[i,0]][AVListIndex[i,1]]= JSDivergence(MomProb[i],datamom[ int((AList.shape[0]-1)/2), int((VList.shape[0]-1)/2), :] );


plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)

plt.imshow(JSDivergenceMatrix, cmap="plasma_r", extent=[VList[0], VList[-1], acc[-1]*accUnit/g, acc[0] *accUnit/g], aspect="auto")
indices_JS = np.where(JSDivergenceMatrix==0)
#plt.plot(AList[indices_JS[1]]*accUnit/g, AList[indices_JS[0]]*accUnit/g)
plt.gca().invert_yaxis();
plt.colorbar()
plt.xlabel("$V_L (E_R)$")
plt.ylabel("Acceleration ($g$)")
plt.title("$D_{JS}(a',V_L'||a=0g, V_L=10E_R)$")
plt.grid(False)

#plt.savefig("JS_curvforlattice.pdf", bbox_inches="tight")