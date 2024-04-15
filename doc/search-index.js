var searchIndex = new Map(JSON.parse('[\
["lattice_evolution",{"doc":"","t":"SSCHCFSSSNNNNNNONNNNOONNNNONNNNNNNNOOONNNNOONNNNONNNNNNNNCJJJJ","n":["FREQ","TOGGLE_INIT","lattice","main","shaking_sequences","Lattice","MASS","N_STATES","N_STEPS","acc_cfi","acc_depth_cfi","acc_qfi","accelerate","borrow","borrow_mut","depth","depth_cfi","depth_qfi","deref","deref_mut","dpsi_a","dpsi_v","drop","fmt","from","from_subset","g","get_acceleration","get_depth","get_dmomentum_da","get_dpsi_a","get_hamiltonian","get_momentum","get_psi","get_time","h0","h1","h2","init","into","is_in_subset","new","psi","q","rk4step","set_depth","set_time","step","time","to_subset","to_subset_unchecked","try_from","try_into","type_id","update","update_d_v","update_da","shaking","L_SHAKING","MP_SHAKING","OPTION1_SHAKING","SP_SHAKING"],"q":[[0,"lattice_evolution"],[5,"lattice_evolution::lattice"],[57,"lattice_evolution::shaking_sequences"],[58,"lattice_evolution::shaking_sequences::shaking"],[62,"core::fmt"],[63,"core::fmt"],[64,"num_complex"],[65,"nalgebra::base::alias"],[66,"core::result"],[67,"core::any"]],"d":["This const is usually used to fix $\\\\omega$ for our shaking …","Decide if we first shake to the +ve or -ve x direction. …","","#Description of code:","","Lattice struct represents an instance of the Shaken …","","","","","","","Accelerate the wavepacket by adding an <em>impulse</em> to $q$ and …","","","lattice depth","","","","","","","","","Returns the argument unchanged.","","acceleration/gravity","Returns $a$ of this Lattice","Returns $V_0$ of this Lattice","","","Returns full Hamiltonian with $\\\\phi=0$ ","","Current Wavefunction of the wavepacket","","The Kinetic part of the Hamiltonian","The $sin(\\\\phi)$ part of the lattice coupling","the $\\\\cos(\\\\phi)$ part of the lattice coupling","","Calls <code>U::from(self)</code>.","","Initializes the Lattice and makes a new instnce Uses given …","current wavefunction of wavepacket moving through lattice","Correction to pure momentum, $(p+q)$ is kinematic momentum","The RK4 step.","Sets $V_0$ of this Lattice","Sets time of this lattice. Used only when we’re manually …","Evolve wavepacket in this lattice given $(A,\\\\omega)$ for …","time","","","","","","update is essentially the derivative to the wavepacket at …","","update_d is essentially the derivative to the state …","","","","",""],"i":[0,0,0,0,0,0,0,0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,0,0,0,0],"f":"```{{}b}`````{df}00{{df}b}{ce{}{}}0`22{hc{}}0``{hb}{{dj}l}{cc{}}4`66{d{{n{f}}}}{d{{n{A`}}}}{d{{Ab{A`}}}}219```{{}h}8{cAd{}}{{ff}d}``{{dffff}b}<<{{dff}b}`{c{{Af{e}}}{}{}}={c{{Ah{e}}}{}{}}0{cAj{}}{{d{n{A`}}fff}{{n{A`}}}}{{d{n{A`}}{n{A`}}fff}{{n{A`}}}}0`````","c":[],"p":[[1,"unit"],[5,"Lattice",5],[1,"f64"],[1,"usize"],[5,"Formatter",62],[8,"Result",62],[8,"DVector",63],[8,"Complex64",64],[8,"DMatrix",63],[1,"bool"],[6,"Option",65],[6,"Result",66],[5,"TypeId",67]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
