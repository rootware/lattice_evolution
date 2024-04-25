halfway = int((len(data_subtracted[0,:])-1)/2)
halfway
p_reshaped = np.concatenate( (data_subtracted[:,halfway+1:], data_subtracted[:,0:halfway+1]), axis=1 )
myfft = np.array([scp.fft.fft(p_reshaped[n]) for n in range(len(data_subtracted))])
myfft_reshaped=np.concatenate( (myfft[:,halfway:], myfft[:,0:halfway+1]), axis=1 )
temp = np.square(np.abs(myfft_reshaped));
myfft_reshaped_rescaled = np.array ( [ temp[m,:]/np.sum(temp[m,:])    for m in range(len(myfft_reshaped)) ] )

plt.figure(figsize = (12, 6))

plt.subplot(121)
im=plt.imshow(myfft_reshaped_rescaled.T, cmap="hot",aspect="0.04", vmax = 0.03, extent=[data[0,2],data[-1,2], -96*2, 96*2])
ax = plt.gca()
#divider = make_axes_locatable(ax)
#cax = divider.append_axes("right", size="25%", pad=0.1)
#plt.colorbar(im, cax=cax)
plt.colorbar(shrink = 0.5)

plt.xlabel("$t\omega_r$")
plt.ylabel("$x (k_L^{-1})$ \n(double check range of $x$)")
plt.title("Wavepacket evolution")
print("Time when TOF starts:\t"+ str( 32*np.pi/11.5) )
y = np.arange(-96*2,96*2,0.1);
plt.plot(np.ones(len(y))*32*np.pi/11.5, y)


plt.subplot(122)
plt.plot(data[:,2],data[:,3], color= "red")
plt.title("DSP shaking with TOF")
plt.ylabel("$\phi(t) = A\sin\omega t$") 
plt.xlabel("$t(\omega_R^{-1})$")
y2 = np.arange(-1.5*np.pi,1.5*np.pi,0.1);
plt.plot(np.ones(len(y2))*32*np.pi/11.5, y2)
plt.tight_layout()

time_to_plot = 17.5 ; # im omega_R inverse
time_index_to_plot = int(9/dt)

fig, ax = plt.subplots();
# animation function. This is called sequentially
image = ax.plot( myfft_reshaped_rescaled[time_index_to_plot, :])[0]
frame_step = 200;
def update(n):
    image.set_ydata( myfft_reshaped_rescaled[time_index_to_plot + frame_step*n, :])
    current_time_stamp =  (n*frame_step + time_index_to_plot)*dt;
    image.axes.set_title(str(int(current_time_stamp)))   
    image.axes.set_xlabel("$x$ indices, need to relabel")
    image.axes.set_ylabel("$P(x,t|a=0,V=10.0)$")
    return (image)
    
ani = animation.FuncAnimation(fig=fig, func=update, frames=int ( (len(time)- time_index_to_plot)/frame_step ), interval=500)
ani.save("animation of time of flight2_shiftedhalfway.gif")
