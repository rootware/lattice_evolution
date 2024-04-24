from matplotlib import ticker
fig=plt.figure(figsize=(4,12))
gs = fig.add_gridspec(7,1 ,hspace=0.05, wspace=0.1)
axs = gs.subplots(sharex='col', sharey='row')


#axs[-1].axis("off");
# Setting the number of ticks 
plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)




minn = np.min([MomProb]);
maxx = np.max([MomProb]);
for i in range(2,9,1):
    ax = axs[i-2,];
    im=ax.imshow(np.reshape(MomProb[:,i], (AList.size,VList.size)), cmap="magma",aspect="auto",extent =[VList[0],VList[-1],AList[0]*accUnit/g,AList[-1]*accUnit/g],  vmin=minn, vmax = maxx, origin="lower",label=str(2*i-10)+"$\hbar k_L$")
    ax.legend(title="$p=$"+str(2*i-10)+"$\hbar k_L$")

    ax.yaxis.set_major_locator(yticks)

    ax.set_xlabel("$V_L$")
   # ax.set_ylabel("$p=$"+str(-10+2*i)+"$\hbar k_L$"+'\n'+"a $(g)$")
    ax.set_ylabel("a$(g)$")
axs[0].set_title("QOC")

plt.grid(False)

# for i in range(3,8,1):
#     ax = axs[i-3,1];
#     im=ax.imshow(np.reshape(MomProbMP[:,i], (AList.size,VList.size)), cmap="magma",aspect="auto",extent =[VList[0],VList[-1],AList[0]*accUnit/g,AList[-1]*accUnit/g],  vmin=minn, vmax = maxx, origin="lower",label=str(2*i-10)+"$\hbar k_L$")
#     ax.legend(title="$p=$"+str(2*i-10)+"$\hbar k_L$")

#     ax.yaxis.set_major_locator(yticks)

#     ax.set_xlabel("$V_L$")
#    # ax.set_ylabel("$p=$"+str(-10+2*i)+"$\hbar k_L$"+'\n'+"a $(g)$")
#     ax.set_ylabel("a$(g)$")

# fig.subplots_adjust(right=0.8)
# cbar_ax = fig.add_axes([0.85, 0.15, 0.05, 0.7])
# #fig.colorbar(im, cax=cbar_ax)
# fig.colorbar(im,  cax = cbar_ax, orientation='vertical')

# axs[0,1].set_title("DSP")
for ax in fig.get_axes():
    ax.label_outer()
plt.grid(False)



#plt.savefig("2dfringes_g_SPMP.pdf", bbox_inches='tight')
