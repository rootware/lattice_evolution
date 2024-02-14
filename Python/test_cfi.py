import numpy as np;
import numpy as np;
import matplotlib.pyplot as plt;
import seaborn as sns;
import seaborn.objects as so;
import Units
from Units import g as g;
from Units import accUnit as accUnit
sns.set_theme()


cfi_data = np.loadtxt("../Data/Rust_Runs/testing_cfi/test_longer_withdepth_withqfi.txt")
cfi_data  = cfi_data[ np.lexsort((cfi_data[:,1],cfi_data[:,0])) ]

plt.plot(cfi_data[:,2]*(accUnit/g), cfi_data[:,4]/cfi_data[250,4], label="$I_{aa}$")
plt.ylabel("$I_{aa}(a)/I_{aa}(a=0.0)$\n $2*D_{JS}(a,a=0)$")
plt.xlabel("Acceleration $(g)$")
plt.legend()
plt.title("$I_{aa}(a)$ normalized by CFI at zero $a$ and $2*JSD(a,a=0)$")