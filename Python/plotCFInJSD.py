import seaborn as sns;
import seaborn.objects as so;
import Units
from Units import g as g;
from Units import accUnit as accUnit
sns.set_theme()

SMALL_SIZE = 16
MEDIUM_SIZE = 16
BIGGER_SIZE = 16

plt.rc('font', size=SMALL_SIZE)          # controls default text sizes
plt.rc('axes', titlesize=SMALL_SIZE)     # fontsize of the axes title
plt.rc('axes', labelsize=MEDIUM_SIZE)    # fontsize of the x and y labels
plt.rc('xtick', labelsize=SMALL_SIZE)    # fontsize of the tick labels
plt.rc('ytick', labelsize=SMALL_SIZE)    # fontsize of the tick labels
plt.rc('legend', fontsize=12)    # legend fontsize
plt.rc('figure', titlesize=BIGGER_SIZE)  # fontsize of the figure title




fig = plt.figure()
ax1 = fig.add_subplot(111)


plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)
yticks = ticker.MaxNLocator(3)

lns1=ax1.plot(cfi_data[:,0]*(accUnit/g), cfi_data[:,2]/cfi_data[50,2], label="CFI")
ax1.set_ylabel("$I_{aa}(a)/I_{aa}(a=0)$")
ax1.set_xlabel("Acceleration $(g)$")
#plt.title("$I_{aa}(a)/I_{aa}(0)$ and $2D_{JS}(a,a=0)$")
plt.legend()

ax2 = ax1.twinx()  # instantiate a second axes that shares the same x-axis
lns2=ax2.plot(AList*Units.accUnit/Units.g, JSDivergenceMatrix[50,:], label="JSD", color= "tab:orange")

ax2.set_ylabel("$D_{JS}(a,a=0)$")
plt.locator_params(axis='both', nbins=3) 
yticks = ticker.MaxNLocator(3)
yticks = ticker.MaxNLocator(3)
plt.title("CFI and JSD vs $a(g)$")
lns = lns1+lns2
labs = [l.get_label() for l in lns]
ax1.legend(lns, labs)

fig.tight_layout()  # otherwise the right y-label is slightly clipped
#plt.savefig("CFInQFI.pdf")
