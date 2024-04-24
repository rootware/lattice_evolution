momindices=np.where(AVListIndex[:,0]==int((AList.shape[0]-1)/2))[0];
momproblist =MomProb[momindices];
indices = AVListIndex[momindices];

lattindices = indices[:, 1];
latt = VList[lattindices]

no_of_values = len(lattindices);
JSDivergenceMatrix=np.zeros((no_of_values, no_of_values)) ;

for i in lattindices:
    for j in lattindices:
        JSDivergenceMatrix [i][j]= JSDivergence(momproblist[i], momproblist[j])

plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)

plt.imshow(JSDivergenceMatrix, cmap="plasma_r", extent=[latt[0], latt[-1], latt[-1], latt[0] ])
indices_JS = np.where(JSDivergenceMatrix==0)
plt.plot(VList[indices_JS[1]], VList[indices_JS[0]])
plt.gca().invert_yaxis();
plt.colorbar()
plt.xlabel("$V_L(E_R)$");
plt.ylabel("$V_L(E_R)$");
plt.title("a = "+ str(AList[50])+"$g$")
plt.grid(False)
#plt.savefig("JS_lattforlattice.pdf", bbox_inches="tight")