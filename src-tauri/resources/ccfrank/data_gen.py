import json

'''
    Source From CCFrank4dblp
    MIT License

    Copyright (c) 2019-2023 WenyanLiu (https://github.com/WenyanLiu/CCFrank4dblp)
'''

# workaround for SIGMOD
ccf_rank_list = '''
A	PACMMOD	Proc. ACM Manag. Data	/journals/pacmmod	/journals/pacmmod/pacmmod

A	TOCS	ACM Transactions on Computer Systems	/journals/tocs	/journals/tocs/tocs 
A	TOS	ACM Transactions on Storage	/journals/tos	/journals/tos/tos
A	TCAD	IEEE Transactions on Computer-Aided Design of Integrated Circuits and Systems	/journals/tcad	/journals/tcad/tcad
A	TC	IEEE Transactions on Computers	/journals/tc	/journals/tc/tc
A	TPDS	IEEE Transactions on Parallel and Distributed Systems	/journals/tpds	/journals/tpds/tpds
A	TACO	ACM Transactions on Architecture and Code Optimization	/journals/taco	/journals/taco/taco
A	TOS	ACM Transactions on Storage	/journals/tos	/journals/tos/tos
A	TCAD	IEEE Transactions on Computer-Aided Design of Integrated Circuits and Systems	/journals/tcad	/journals/tcad/tcad
A	TC	IEEE Transactions on Computers	/journals/tc	/journals/tc/tc
A	TPDS	IEEE Transactions on Parallel and Distributed Systems	/journals/tpds	/journals/tpds/tpds
A	TACO	ACM Transactions on Architecture and Code Optimization	/journals/taco	/journals/taco/taco
B	TAAS	ACM Transactions on Autonomous and Adaptive Systems	/journals/taas	/journals/taas/taas
B	TODAES	ACM Transactions on Design Automation of Electronic Systems	/journals/todaes	/journals/todaes/todaes
B	TECS	ACM Transactions on Embedded Computing Systems	/journals/tecs	/journals/tecs/tecs
B	TRETS	ACM Transactions on Reconfigurable Technology and Systems	/journals/trets	/journals/trets/trets
B	TVLSI	IEEE Transactions on Very Large Scale Integration (VLSI) Systems	/journals/tvlsi	/journals/tvlsi/tvlsi
B	JPDC	Journal of Parallel and Distributed Computing	/journals/jpdc	/journals/jpdc/jpdc
B	JSA	Journal of Systems Architecture: Embedded Software Design	/journals/jsa	/journals/jsa/jsa
B		Parallel Computing	/conf/parco	/conf/parco/parco
B		Parallel Computing	/journals/pc	/journals/pc/pc
B		Performance Evaluation: An International Journal	/journals/pe	/journals/pe/pe
B	TCC	IEEE Transactions on Cloud Computing	/journals/tcc	/journals/tcc/tcc
C	JETC	ACM Journal on Emerging Technologies in Computing Systems	/journals/jetc	/journals/jetc/jetc
C		Concurrency and Computation: Practice and Experience	/journals/concurrency	/journals/concurrency/concurrency
C	DC	Distributed Computing	/journals/dc	/journals/dc/dc
C	FGCS	Future Generation Computer Systems	/journals/fgcs	/journals/fgcs/fgcs
C	Integration	Integration, the VLSI Journal	/journals/integration	/journals/integration/integration
C	JETTA	Journal of Electronic Testing-Theory and Applications	/journals/et	/journals/et/et
C	JGC	Journal of Grid computing	/journals/grid	/journals/grid/grid
C	RTS	Real-Time Systems	/journals/rts	/journals/rts/rts
C	TJSC	The Journal of Supercomputing	/journals/tjs	/journals/tjs/tjs
C	TCASI	IEEE Transactions on Circuits and Systems I: Regular Papers	/journals/tcasI	/journals/tcasI/tcasI
C	CCF-THPC	CCF Transactions on High Performance Computing	/journals/ccfthpc	/journals/ccfthpc/ccfthpc
C	TSUSC	IEEE Transactions on Sustainable Computing	/journals/tsusc	/journals/tsusc/tsusc
A	PPoPP	ACM SIGPLAN Symposium on Principles & Practice of Parallel Programming	/conf/ppopp	/conf/ppopp/ppopp
A	FAST	USENIX Conference on File and Storage Technologies	/conf/fast	/conf/fast/fast
A	DAC	Design Automation Conference	/conf/dac	/conf/dac/dac
A	HPCA	IEEE International Symposium on High Performance Computer Architecture	/conf/hpca	/conf/hpca/hpca
A	MICRO	IEEE/ACM International Symposium on Microarchitecture	/conf/micro	/conf/micro/micro
A	SC	International Conference for High Performance Computing, Networking, Storage, and Analysis	/conf/sc	/conf/sc/sc
A	ASPLOS	International Conference on Architectural Support for Programming Languages and Operating Systems	/conf/asplos	/conf/asplos/asplos
A	ISCA	International Symposium on Computer Architecture	/conf/isca	/conf/isca/isca
A	ACM SIGOPS ATC	ACM SIGOPS Annual Technical Conference	/conf/usenix	/conf/usenix/usenix
A	EuroSys	European Conference on Computer Systems	/conf/eurosys	/conf/eurosys/eurosys
A	HPDC	The International ACM Symposium on High-Performance Parallel and Distributed Computing	/conf/hpdc	/conf/hpdc/hpdc
B	SoCC	ACM Symposium on Cloud Computing	/conf/cloud	/conf/cloud/socc
B	SPAA	ACM Symposium on Parallelism in Algorithms and Architectures	/conf/spaa	/conf/spaa/spaa
B	PODC	ACM Symposium on Principles of Distributed Computing	/conf/podc	/conf/podc/podc
B	FPGA	ACM/SIGDA International Symposium on Field-Programmable Gate Arrays	/conf/fpga	/conf/fpga/fpga
B	CGO	The International Symposium on Code Generation and Optimization	/conf/cgo	/conf/cgo/cgo
B	DATE	Design, Automation & Test in Europe	/conf/date	/conf/date/date
B	HOT CHIPS	Hot Chips: A Symposium on High Performance Chips	/conf/hotchips	/conf/hotchips/hotchips
B	CLUSTER	IEEE International Conference on Cluster Computing	/conf/cluster	/conf/cluster/cluster
B	ICCD	International Conference on Computer Design	/conf/iccd	/conf/iccd/iccd
B	ICCAD	International Conference on Computer-Aided Design	/conf/iccad	/conf/iccad/iccad
B	ICDCS	IEEE International Conference on Distributed Computing Systems	/conf/icdcs	/conf/icdcs/icdcs
B	CODEISSS	International Conference on Hardware/Software Co-design and System Synthesis	/conf/codes	/conf/codes/codes
B	CODEISSS	International Conference on Hardware/Software Co-design and System Synthesis	/conf/codesisss	/conf/codes/codesisss
B	HiPEAC	International Conference on High Performance and Embedded Architectures and Compilers	/conf/hipeac	/conf/hipeac/hipeac
B	SIGMETRICS	International Conference on Measurement and Modeling of Computer Systems	/conf/sigmetrics	/conf/sigmetrics/sigmetrics
B	PACT	International Conference on Parallel Architectures and Compilation Techniques	/conf/IEEEpact	/conf/IEEEpact/pact
B	PACT	International Conference on Parallel Architectures and Compilation Techniques	/conf/IEEEpact	/conf/IEEEpact/IEEEpact
B	ICPP	International Conference on Parallel Processing	/conf/icpp	/conf/icpp/icpp
B	ICS	International Conference on Supercomputing	/conf/ics	/conf/ics/ics
B	VEE	International Conference on Virtual Execution Environments	/conf/vee	/conf/vee/vee
B	IPDPS	IEEE International Parallel & Distributed Processing Symposium	/conf/ipps	/conf/ipps/ipdps
B	Performance	International Symposium on Computer Performance, Modeling, Measurements and Evaluation	/conf/performance	/conf/performance/performance
B	ITC	International Test Conference	/conf/itc	/conf/itc/itc
B	LISA	Large Installation System Administration Conference	/conf/lisa	/conf/lisa/lisa
B	MSST	Mass Storage Systems and Technologies	/conf/mss	/conf/mss/msst
B	RTAS	IEEE Real-Time and Embedded Technology and Applications Symposium	/conf/rtas	/conf/rtas/rtas
B	Euro-Par	European Conference on Parallel and Distributed Computing	/conf/europar	/conf/europar/europar
B	ISCAS	IEEE International Symposium on Circuits and Systems	/conf/iscas	/conf/iscas/iscas
C	CF	ACM International Conference on Computing Frontiers	/conf/cf	/conf/cf/cf
C	SYSTOR	ACM International Systems and Storage Conference	/conf/systor	/conf/systor/systor
C	NOCS	ACM/IEEE International Symposium on Networks-on-Chip	/conf/nocs	/conf/nocs/nocs
C	ASAP	IEEE International Conference on Application-Specific Systems, Architectures, and Processors	/conf/asap	/conf/asap/asap
C	ASP-DAC	Asia and South Pacific Design Automation Conference	/conf/aspdac	/conf/aspdac/aspdac
C	ETS	IEEE European Test Symposium	/conf/ets	/conf/ets/ets
C	FPL	International Conference on Field-Programmable Logic and Applications	/conf/fpl	/conf/fpl/fpl
C	FCCM	IEEE Symposium on Field-Programmable Custom Computing Machines	/conf/fccm	/conf/fccm/fccm
C	GLSVLSI	Great Lakes Symposium on VLSI	/conf/glvlsi	/conf/glvlsi/glvlsi
C	ATS	IEEE Asian Test Symposium	/conf/ats	/conf/ats/ats
C	HPCC	IEEE International Conference on High Performance Computing and Communications	/conf/hpcc	/conf/hpcc/hpcc
C	HiPC	IEEE International Conference on High Performance Computing, Data and Analytics	/conf/hipc	/conf/hipc/hipc
C	MASCOTS	International Symposium on Modeling, Analysis, and Simulation of Computer and Telecommunication Systems	/conf/mascots	/conf/mascots/mascots
C	ISPA	IEEE International Symposium on Parallel and Distributed Processing with Applications	/conf/ispa	/conf/ispa/ispa
C	CCGRID	IEEE/ACM International Symposium on Cluster, Cloud and Grid Computing	/conf/ccgrid	/conf/ccgrid/ccgrid
C	NPC	IFIP International Conference on Network and Parallel Computing	/conf/npc	/conf/npc/npc
C	ICA3PP	International Conference on Algorithms and Architectures for Parallel Processing	/conf/ica3pp	/conf/ica3pp/ica3pp
C	CASES	International Conference on Compilers, Architectures, and Synthesis for Embedded Systems	/conf/cases	/conf/cases/cases
C	FPT	International Conference on Field-Programmable Technology	/conf/fpt	/conf/icfpt/icfpt
C	FPT	International Conference on Field-Programmable Technology	/conf/fpt	/conf/fpt/fpt
C	ICPADS	International Conference on Parallel and Distributed Systems	/conf/icpads	/conf/icpads/icpads
C	ISLPED	International Symposium on Low Power Electronics and Design	/conf/islped	/conf/islped/islped
C	ISPD	International Symposium on Physical Design	/conf/ispd	/conf/ispd/ispd
C	HOTI	IEEE Symposium on High-Performance Interconnects	/conf/hoti	/conf/hoti/hoti
C	VTS	IEEE VLSI Test Symposium	/conf/vts	/conf/vts/vts
C	ITC-Asia	International Test Conference in Asia	/conf/itc-asia	/conf/itc-asia/itc-asia
C	SEC	ACM/IEEE Symposium on Edge Computing	/conf/ieeesec	/conf/ieeesec/sec
C	NAS	International Conference on Networking, Architecture and Storages	/conf/nas	/conf/nas/nas
C	HotStorage	HotStorage	/conf/hotstorage	/conf/hotstorage/hotstorage
C	APPT	International Symposium on Advanced Parallel Processing Technology	/conf/appt	/conf/appt/appt
C	JCC	International Conference on JointCloud Computing		
A	JSAC	IEEE Journal on Selected Areas in Communications	/journals/jsac	/journals/jsac/jsac
A	TMC	IEEE Transactions on Mobile Computing	/journals/tmc	/journals/tmc/tmc
A	TON	IEEE Transactions on Networking	/journals/ton	/journals/ton/ton
B	TOIT	ACM Transactions on Internet Technology	/journals/toit	/journals/toit/toit
B	TOMM	ACM Transactions on Multimedia Computing, Communications and Applications	/journals/tomccap	/journals/tomccap/tomccap
B	TOSN	ACM Transactions on Sensor Networks	/journals/tosn	/journals/tosn/tosn
B	CN	Computer Networks	/journals/cn	/journals/cn/cn
B	TCOM	IEEE Transactions on Communications	/journals/tcom	/journals/tcom/tcom
B	TWC	IEEE Transactions on Wireless Communications	/journals/twc	/journals/twc/twc
C		Ad hoc Networks	/journals/adhoc	/journals/adhoc/adhoc
C	CC	Computer Communications	/journals/comcom	/journals/comcom/comcom
C	TNSM	IEEE Transactions on Network and Service Management	/journals/tnsm	/journals/tnsm/tnsm
C		IET Communications	/journals/iet-com	/journals/iet-com/iet-com
C	JNCA	Journal of Network and Computer Applications	/journals/jnca	/journals/jnca/jnca
C	MONET	Mobile Networks and Applications	/journals/monet	/journals/monet/monet
C		Networks	/journals/networks	/journals/networks/networks
C	PPNA	Peer-to-Peer Networking and Applications	/journals/ppna	/journals/ppna/ppna
C	WCMC	Wireless Communications and Mobile Computing	/journals/wicomm	/journals/wicomm/wicomm
C		Wireless Networks	/journals/winet	/journals/winet/winet
C	IOT	IEEE Internet of Things Journal	/journals/iotj	/journals/iotj/iotj
C	TIOT	ACM Transactions on Internet of Things /journals/tiot /journals/tiot/tiot
A	SIGCOMM	ACM International Conference on Applications, Technologies, Architectures, and Protocols for Computer Communication	/conf/sigcomm	/conf/sigcomm/sigcomm
A	MobiCom	ACM International Conference on Mobile Computing and Networking	/conf/mobicom	/conf/mobicom/mobicom
A	INFOCOM	IEEE International Conference on Computer Communications	/conf/infocom	/conf/infocom/infocom
A	NSDI	Symposium on Network System Design and Implementation	/conf/nsdi	/conf/nsdi/nsdi
B	SenSys	ACM Conference on Embedded Networked Sensor Systems	/conf/sensys	/conf/sensys/sensys
B	CoNEXT	ACM International Conference on Emerging Networking Experiments and Technologies	/conf/conext	/conf/conext/conext
B	SECON	IEEE International Conference on Sensing, Communication, and Networking	/conf/secon	/conf/secon/secon
B	IPSN	International Conference on Information Processing in Sensor Networks	/conf/ipsn	/conf/ipsn/ipsn
B	MobiSys	ACM International Conference on Mobile Systems, Applications, and Services	/conf/mobisys	/conf/mobisys/mobisys
B	ICNP	IEEE International Conference on Network Protocols	/conf/icnp	/conf/icnp/icnp
B	MobiHoc	International Symposium on Theory, Algorithmic Foundations, and Protocol Design for Mobile Networks and Mobile Computing	/conf/mobihoc	/conf/mobihoc/mobihoc
B	NOSSDAV	International Workshop on Network and Operating System Support for Digital Audio and Video	/conf/nossdav	/conf/nossdav/nossdav
B	IWQoS	IEEE/ACM International Workshop on Quality of Service	/conf/iwqos	/conf/iwqos/iwqos
B	IMC	ACM Internet Measurement Conference	/conf/imc	/conf/imc/imc
C	ANCS	ACM/IEEE Symposium on Architectures for Networking and Communication Systems	/conf/ancs	/conf/ancs/ancs
C	APNOMS	Asia-Pacific Network Operations and Management Symposium	/conf/apnoms	/conf/apnoms/apnoms
C	FORTE	International Conference on Formal Techniques for Distributed Objects, Components, and Systems	/conf/forte	/conf/forte/forte
C	LCN	IEEE Conference on Local Computer Networks	/conf/lcn	/conf/lcn/lcn
C	GLOBECOM	IEEE Global Communications Conference	/conf/globecom	/conf/globecom/globecom
C	ICC	IEEE International Conference on Communications	/conf/icc	/conf/icc/icc
C	ICCCN	IEEE International Conference on Computer Communications and Networks	/conf/icccn	/conf/icccn/icccn
C	MASS	IEEE International Conference on Mobile Adhoc and Sensor Systems	/conf/mass	/conf/mass/mass
C	P2P	IEEE International Conference on P2P Computing	/conf/p2p	/conf/p2p/p2p
C	IPCCC	IEEE International Performance Computing and Communications Conference	/conf/ipccc	/conf/ipccc/ipccc
C	WoWMoM	IEEE International Symposium on a World of Wireless, Mobile and Multimedia Networks	/conf/wowmom	/conf/wowmom/wowmom
C	ISCC	IEEE Symposium on Computers and Communications	/conf/iscc	/conf/iscc/iscc
C	WCNC	IEEE Wireless Communications and Networking Conference	/conf/wcnc	/conf/wcnc/wcnc
C	Networking	IFIP International Conferences on Networking	/conf/networking	/conf/networking/networking
C	IM	IFIP/IEEE International Symposium on Integrated Network Management	/conf/im	/conf/im/im
C	MSN	International Conference on Mobility, Sensing and Networking	/conf/msn	/conf/msn/msn
C	MSWiM	International Conference on Modeling, Analysis and Simulation of Wireless and Mobile Systems	/conf/mswim	/conf/mswim/mswim
C	WASA	The International Conference on Wireless Artificial Intelligent Computing Systems and Applications	/conf/wasa	/conf/wasa/wasa
C	HotNets	ACM The Workshop on Hot Topics in Networks	/conf/hotnets	/conf/hotnets/hotnets
C	APNet	Asia-Pacific Workshop on Networking	/conf/apnet	/conf/apnet/apnet
A	TDSC	IEEE Transactions on Dependable and Secure Computing	/journals/tdsc	/journals/tdsc/tdsc
A	TIFS	IEEE Transactions on Information Forensics and Security	/journals/tifs	/journals/tifs/tifs
A		Journal of Cryptology	/journals/joc	/journals/joc/joc
B	TOPS	ACM Transactions on Privacy and Security	/journals/tissec	/journals/tissec/tissec
B		Computers & Security	/journals/compsec	/journals/compsec/compsec
B		Designs, Codes and Cryptography	/journals/dcc	/journals/dcc/dcc
B	JCS	Journal of Computer Security	/journals/jcs	/journals/jcs/jcs
B	Cybersecurity	Cybersecurity	/journals/cybersec	/journals/cybersec/cybersec
C	CLSR	Computer Law & Security Review	/journals/clsr	/journals/clsr/clsr
C		EURASIP Journal on Information Security	/journals/ejisec	/journals/ejisec/ejisec
C		IET Information Security	/journals/iet-ifs	/journals/iet-ifs/iet-ifs
C	IMCS	Information and Computer Security	/journals/imcs	/journals/imcs/imcs
C	IJICS	International Journal of Information and Computer Security	/journals/ijics	/journals/ijics/ijics
C	IJISP	International Journal of Information Security and Privacy	/journals/ijisp	/journals/ijisp/ijisp
C	JISA	Journal of Information Security and Applications	/journals/istr	/journals/istr/istr
C	SCN	Security and Communication Networks	/journals/scn	/journals/scn/scn
C	HCC	High-Confidence Computing	/journals/hcc	/journals/hcc/hcc
A	CCS	ACM Conference on Computer and Communications Security	/conf/ccs	/conf/ccs/ccs
A	EUROCRYPT	International Conference on the Theory and Applications of Cryptographic Techniques	/conf/eurocrypt	/conf/eurocrypt/eurocrypt
A	S&P	IEEE Symposium on Security and Privacy	/conf/sp	/conf/sp/sp
A	CRYPTO	International Cryptology Conference	/conf/crypto	/conf/crypto/crypto
A	USENIX Security	USENIX Security Symposium	/conf/uss	/conf/uss/uss
A	NDSS	Network and Distributed System Security Symposium	/conf/ndss	/conf/ndss/ndss
B	ACSAC	Annual Computer Security Applications Conference	/conf/acsac	/conf/acsac/acsac
B	ASIACRYPT	Annual International Conference on the Theory and Application of Cryptology and Information Security	/conf/asiacrypt	/conf/asiacrypt/asiacrypt
B	ESORICS	European Symposium on Research in Computer Security	/conf/esorics	/conf/esorics/esorics
B	FSE	Fast Software Encryption	/conf/fse	/conf/fse/fse
B	CSFW	IEEE Computer Security Foundations Workshop	/conf/csfw	/conf/csfw/csfw
B	SRDS	IEEE International Symposium on Reliable Distributed Systems	/conf/srds	/conf/srds/srds
B	CHES	International Conference on Cryptographic Hardware and Embedded Systems	/conf/ches	/conf/ches/ches
B	DSN	International Conference on Dependable Systems and Networks	/conf/dsn	/conf/dsn/dsn
B	RAID	International Symposium on Recent Advances in Intrusion Detection	/conf/raid	/conf/raid/raid
B	PKC	International Workshop on Practice and Theory in Public Key Cryptography	/conf/pkc	/conf/pkc/pkc
B	TCC	Theory of Cryptography Conference	/conf/tcc	/conf/tcc/tcc
C	WiSec	ACM Conference on Security and Privacy in Wireless and Mobile Networks	/conf/wisec	/conf/wisec/wisec
C	SACMAT	ACM Symposium on Access Control Models and Technologies	/conf/sacmat	/conf/sacmat/sacmat
C	DRM	ACM Workshop on Digital Rights Management	/conf/drm	/conf/drm/drm
C	IH&MMSec	ACM Workshop on Information Hiding and Multimedia Security	/conf/ih	/conf/ih/ihmmsec
C	IH&MMSec	ACM Workshop on Information Hiding and Multimedia Security	/conf/ih	/conf/ih/ih
C	ACNS	Applied Cryptography and Network Security	/conf/acns	/conf/acns/acns
C	AsiaCCS	Asia Conference on Computer and Communications Security	/conf/asiaccs	/conf/ccs/asiaccs
C	AsiaCCS	Asia Conference on Computer and Communications Security	/conf/asiaccs	/conf/asiaccs/asiaccs
C	ACISP	Australasia Conference on Information Security and Privacy	/conf/acisp	/conf/acisp/acisp
C	CT-RSA	The Cryptographer's Track at RSA Conference	/conf/ctrsa	/conf/ctrsa/ctrsa
C	DIMVA	Conference on Detection of Intrusions and Malware & Vulnerability Assessment	/conf/dimva	/conf/dimva/dimva
C	DFRWS	Digital Forensic Research Workshop	/conf/dfrws	/conf/dfrws/dfrws
C	FC	Financial Cryptography and Data Security	/conf/fc	/conf/fc/fc
C	TrustCom	IEEE International Conference on Trust,Security and Privacy in Computing and Communications	/conf/trustcom	/conf/trustcom/trustcom
C	SEC	IFIP International Information Security Conference	/conf/sec	/conf/sec/sec
C	IFIP WG 11.9	IFIP WG 11.9 International Conference on Digital Forensics	/conf/ifip11-9	/conf/ifip11-9/df
C	ISC	Information Security Conference	/conf/isw	/conf/isw/isc
C	ISC	Information Security Conference	/conf/isw	/conf/isw/isw
C	ICDF2C	International Conference on Digital Forensics & Cyber Crime	/conf/icdf2c	/conf/icdf2c/icdf2c
C	ICICS	International Conference on Information and Communications Security	/conf/icics	/conf/icics/icics
C	SecureComm	International Conference on Security and Privacy in Communication Networks	/conf/securecomm	/conf/securecomm/securecomm
C	NSPW	New Security Paradigms Workshop	/conf/nspw	/conf/nspw/nspw
C	PAM	Passive and Active Measurement Conference	/conf/pam	/conf/pam/pam
C	PETS	Privacy Enhancing Technologies Symposium	/conf/pet	/conf/pet/pets
C	PETS	Privacy Enhancing Technologies Symposium	/conf/pet	/conf/pet/pet
C	SAC	Selected Areas in Cryptography	/conf/sacrypt	/conf/sacrypt/sacrypt
C	SOUPS	Symposium On Usable Privacy and Security	/conf/soups	/conf/soups/soups
C	HotSec	USENIX Workshop on Hot Topics in Security	/conf/uss/	/conf/uss/hotsec
C	EuroS&P	IEEE European Symposium on Security and Privacy	/conf/eurosp	/conf/eurosp/eurosp
C	Inscrypt	International Conference on Information Security and Cryptology	/conf/icisc	/conf/icisc/icisc
C	Inscrypt	International Conference on Information Security and Cryptology	/conf/cisc	/conf/cisc/inscrypt
C	CODASPY	Conference on Data and Application Security and Privacy	/conf/codaspy	/conf/codaspy/codaspy
C	BlockSys	International Conference on Blockchain, Artificial Intelligence, and Trustworthy Systems	/conf/blocksys	/conf/blocksys/blocksys
C	CSCloud	International Conference on Cyber Security and Cloud Computing	/conf/cscloud	/conf/cscloud/cscloud
A	TOPLAS	ACM Transactions on Programming Languages and Systems	/journals/toplas	/journals/toplas/toplas
A	TOSEM	ACM Transactions on Software Engineering and Methodology	/journals/tosem	/journals/tosem/tosem
A	TSE	IEEE Transactions on Software Engineering	/journals/tse	/journals/tse/tse
A	TSC	IEEE Transactions on Services Computing	/journals/tsc	/journals/tsc/tsc
B	ASE	Automated Software Engineering	/journals/ase	/journals/ase/ase
B	ESE	Empirical Software Engineering	/journals/ese	/journals/ese/ese
B	IETS	IET Software	/journals/iee	/journals/iee/iee-s
B	IETS	IET Software	/journals/iet-sen	/journals/iet-sen/iet-sen
B	IST	Information and Software Technology	/journals/infsof	/journals/infsof/infsof
B	JFP	Journal of Functional Programming	/journals/jfp	/journals/jfp/jfp
B		Journal of Software: Evolution and Process	/journals/smr	/journals/smr/smr
B	JSS	Journal of Systems and Software	/journals/jss	/journals/jss/jss
B	RE	Requirements Engineering	/journals/re	/journals/re/re
B	SCP	Science of Computer Programming	/journals/scp	/journals/scp/scp
B	SoSyM	Software and Systems Modeling	/journals/sosym	/journals/sosym/sosym
B	STVR	Software Testing, Verification and Reliability	/journals/stvr	/journals/stvr/stvr
B	SPE	Software: Practice and Experience	/journals/spe	/journals/spe/spe
C	CL	Computer Languages, Systems and Structures	/journals/cl	/journals/cl/cl
C	IJSEKE	International Journal of Software Engineering and Knowledge Engineering	/journals/ijseke	/journals/ijseke/ijseke
C	STTT	International Journal of Software Tools for Technology Transfer	/journals/sttt	/journals/sttt/sttt
C	JLAMP	Journal of Logical and Algebraic Methods in Programming	/journals/jlap	/journals/jlap/jlap
C	JLAMP	Journal of Logical and Algebraic Methods in Programming	/journals/jlap	/journals/jlp/jlp
C	JWE	Journal of Web Engineering	/journals/jwe	/journals/jwe/jwe
C	SOCA	Service Oriented Computing and Applications	/journals/soca	/journals/soca/soca
C	SQJ	Software Quality Journal	/journals/sqj	/journals/sqj/sqj
C	TPLP	Theory and Practice of Logic Programming	/journals/tplp	/journals/tplp/tplp
C	PACM PL	Proceedings of the ACM on Programming Languages	/journals/pacmpl	/journals/pacmpl/pacmpl
A	PLDI	ACM SIGPLAN Conference on Programming Language Design and Implementation	/conf/pldi	/conf/pldi/pldi
A	POPL	ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages	/conf/popl	/conf/popl/popl
A	FSE	ACM International Conference on the Foundations of Software Engineering	/conf/sigsoft	/conf/sigsoft/fse
A	SOSP	ACM Symposium on Operating Systems Principles	/conf/sosp	/conf/sosp/sosp
A	OOPSLA	Conference on Object-Oriented Programming Systems, Languages,and Applications	/conf/oopsla	/conf/oopsla/oopsla
A	ASE	International Conference on Automated Software Engineering	/conf/kbse	/conf/kbse/ase
A	ASE	International Conference on Automated Software Engineering	/conf/kbse	/conf/kbse/kbse
A	ICSE	International Conference on Software Engineering	/conf/icse	/conf/icse/icse
A	ISSTA	International Symposium on Software Testing and Analysis	/conf/issta	/conf/issta/issta
A	OSDI	USENIX Symposium on Operating Systems Design and Implementation	/conf/osdi	/conf/osdi/osdi
A	FM	International Symposium on Formal Methods	/conf/fm	/conf/fm/fm
B	ECOOP	European Conference on Object-Oriented Programming	/conf/ecoop	/conf/ecoop/ecoop
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/esop/esop
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/fase/fase
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/fossacs/fossacs
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/tacas/tacas
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/post/post
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/spin/spin
B	ETAPS	European Joint Conferences on Theory and Practice of Software	/conf/etaps	/conf/etaps/etaps
B	ICPC	IEEE International Conference on Program Comprehension	/conf/iwpc	/conf/iwpc/icpc
B	ICPC	IEEE International Conference on Program Comprehension	/conf/iwpc	/conf/iwpc/iwpc
B	RE	IEEE International Requirements Engineering Conference	/conf/re	/conf/re/re
B	RE	IEEE International Requirements Engineering Conference	/conf/re	/conf/icre/icre
B	CAiSE	International Conference on Advanced Information Systems Engineering	/conf/caise	/conf/caise/caise
B	ICFP	International Conference on Function Programming	/conf/icfp	/conf/icfp/icfp
B	LCTES	ACM SIGPLAN/SIGBED International Conference on Languages, Compilers and Tools for Embedded Systems	/conf/lctrts	/conf/lctrts/lctes
B	MoDELS	ACM/IEEE International Conference on Model Driven Engineering Languages and Systems	/conf/models	/conf/models/models
B	CP	International Conference on Principles and Practice of Constraint Programming	/conf/cp	/conf/cp/cp
B	ICSOC	International Conference on Service Oriented Computing	/conf/icsoc	/conf/icsoc/icsoc
B	SANER	International Conference on Software Analysis, Evolution, and Reengineering	/conf/wcre	/conf/wcre/saner
B	SANER	International Conference on Software Analysis, Evolution, and Reengineering	/conf/wcre	/conf/wcre/wcre
B	ICSME	International Conference on Software Maintenance and Evolution	/conf/icsm	/conf/icsm/icsme
B	ICSME	International Conference on Software Maintenance and Evolution	/conf/icsm	/conf/icsm/icsm
B	VMCAI	International Conference on Verification, Model Checking and Abstract Interpretation	/conf/vmcai	/conf/vmcai/vmcai
B	ICWS	IEEE International Conference on Web Services	/conf/icws	/conf/icws/icws
B	Middleware	International Middleware Conference	/conf/middleware	/conf/middleware/middleware
B	SAS	International Static Analysis Symposium	/conf/sas	/conf/sas/sas
B	ESEM	International Symposium on Empirical Software Engineering and Measurement	/conf/esem	/conf/esem/esem
B	ISSRE	International Symposium on Software Reliability Engineering	/conf/issre	/conf/issre/issre
B	HotOS	USENIX Workshop on Hot Topics in Operating Systems	/conf/hotos	/conf/hotos/hotos
B	CC	International Conference on Compiler Construction	/conf/cc	/conf/cc/cc
C	PEPM	ACM SIGPLAN Workshop on Partial Evaluation and Program Manipulation	/conf/pepm	/conf/pepm/pepm
C	PASTE	ACMSIGPLAN-SIGSOFT Workshop on Program Analysis for Software Tools and Engineering	/conf/paste	/conf/paste/paste
C	APLAS	Asian Symposium on Programming Languages and Systems	/conf/aplas	/conf/aplas/aplas
C	APSEC	Asia-Pacific Software Engineering Conference	/conf/apsec	/conf/apsec/apsec
C	EASE	International Conference on Evaluation and Assessment in Software Engineering	/conf/ease	/conf/ease/ease
C	ICECCS	IEEE International Conference on Engineering of Complex Computer Systems	/conf/iceccs	/conf/iceccs/iceccs
C	ICST	IEEE International Conference on Software Testing, Verification and Validation	/conf/icst	/conf/icst/icst
C	ISPASS	IEEE International Symposium on Performance Analysis of Systems and Software	/conf/ispass	/conf/ispass/ispass
C	SCAM	IEEE International Working Conference on Source Code Analysis and Manipulation	/conf/scam	/conf/scam/scam
C	COMPSAC	International Computer Software and Applications Conference	/conf/compsac	/conf/compsac/compsac
C	ICFEM	International Conference on Formal Engineering Methods	/conf/icfem	/conf/icfem/icfem
C	SSE	IEEE International Conference on Software Services Engineering	/conf/IEEEscc	/conf/IEEEscc/scc
C	ICSSP	International Conference on Software and System Process	/conf/ispw	/conf/ispw/icssp
C	ICSSP	International Conference on Software and System Process	/conf/ispw	/conf/ispw/icsp
C	SEKE	International Conference on Software Engineering and Knowledge Engineering	/conf/seke	/conf/seke/seke
C	QRS	International Conference on Software Quality, Reliability and Security	/conf/qrs	/conf/qrs/qrs
C	ICSR	International Conference on Software Reuse	/conf/icsr	/conf/icsr/icsr
C	ICWE	International Conference on Web Engineering	/conf/icwe	/conf/icwe/icwe
C	SPIN	International SPIN Workshop on Model Checking of Software	/conf/spin	/conf/spin/spin
C	ATVA	International Symposium on Automated Technology for Verification and Analysis	/conf/atva	/conf/atva/atva
C	LOPSTR	International Symposium on Logic-based Program Synthesis and Transformation	/conf/lopstr	/conf/lopstr/lopstr
C	TASE	Theoretical Aspects of Software Engineering Conference	/conf/tase	/conf/tase/tase
C	MSR	Mining Software Repositories	/conf/msr	/conf/msr/msr
C	REFSQ	Requirements Engineering: Foundation for Software Quality	/conf/refsq	/conf/refsq/refsq
C	WICSA	Working IEEE/IFIP Conference on Software Architecture	/conf/wicsa	/conf/wicsa/wicsa
C	Internetware	Asia-Pacific Symposium on Internetware	/conf/internetware	/conf/internetware/internetware
C	RV	International Conference on Runtime Verification	/conf/rv	/conf/rv/rv
C	MEMOCODE	International Conference on Formal Methods and Models for Co-Design	/conf/memocode	/conf/memocode/memocode
A 	TODS	ACM Transactions on Database Systems	/journals/tods	/journals/tods/tods
A 	TOIS	ACM Transactions on Information Systems	/journals/tois	/journals/tois/tois
A 	TKDE	IEEE Transactions on Knowledge and Data Engineering	/journals/tkde	/journals/tkde/tkde
A 	VLDBJ	The VLDB Journal	/journals/vldb	/journals/vldb/vldb
B	TKDD	ACM Transactions on Knowledge Discovery from Data	/journals/tkdd	/journals/tkdd/tkdd
B	TWEB	ACM Transactions on the Web	/journals/tweb	/journals/tweb
B	AEI	Advanced Engineering Informatics	/journals/aei	/journals/aei/aei
B	DKE	Data & Knowledge Engineering	/journals/dke	/journals/dke/dke
B	DMKD	Data Mining and Knowledge Discovery	/journals/datamine	/journals/datamine/datamine
B	EJIS	European Journal of Information Systems	/journals/ejis	/journals/ejis/ejis
B		GeoInformatica	/journals/geoinformatica	/journals/geoinformatica/geoinformatica
B	IPM	Information Processing and Management	/journals/ipm	/journals/ipm/ipm
B		Information Sciences	/journals/isci	/journals/isci/isci
B	IS	Information Systems	/journals/is	/journals/is/is
B	JASIST	Journal of the Association for Information Science and Technology	/journals/jasis	/journals/jasis/jasis
B	JWS	Journal of Web Semantics	/journals/ws	/journals/ws/ws
B	KAIS	Knowledge and Information Systems	/journals/kais	/journals/kais/kais
B	DSE	Data Science and Engineering	/journals/dase	/journals/dase/dase
C	DPD	Distributed and Parallel Databases	/journals/dpd	/journals/dpd/dpd
C	I&M	Information & Management	/journals/iam	/journals/iam/iam
C	IPL	Information Processing Letters	/journals/ipl	/journals/ipl/ipl
C		Discover Computing	/journals/ir	/journals/ir/ir
C	IJCIS	International Journal of Cooperative Information Systems	/journals/ijcis	/journals/ijcis/ijcis
C	IJGIS	International Journal of Geographical Information Science	/journals/gis	/journals/gis/gis
C	IJIS	International Journal of Intelligent Systems	/journals/ijis	/journals/ijis/ijis
C	IJKM	International Journal of Knowledge Management	/journals/ijkm	/journals/ijkm/ijkm
C	IJSWIS	International Journal on Semantic Web and Information Systems	/journals/ijswis	/journals/ijswis/ijswis
C	JCIS	Journal of Computer Information Systems	/journals/jcis	/journals/jcis/jcis
C	JDM	Journal of Database Management	/journals/jdm	/journals/jdm/jdm
C	JGITM	Journal of Global Information Technology Management	/journals/jgim	/journals/jgim/jgim
C	JIIS	Journal of Intelligent Information Systems	/journals/jiis	/journals/jiis/jiis
C	JSIS	Journal of Strategic Information Systems	/journals/jsis	/journals/jsis/jsis
C	TIST	ACM Transactions on Intelligent Systems and Technology	/journals/tist	/journals/tist/tist
C	TORS	ACM Transactions on Recommender Systems	/journals/tors	/journals/tors/tors
A	SIGMOD	ACM SIGMOD Conference	/conf/sigmod	/conf/sigmod/sigmod
A	SIGKDD	ACM SIGKDD Conference on Knowledge Discovery and Data Mining	/conf/kdd	/conf/kdd/kdd
A	ICDE	IEEE International Conference on Data Engineering	/conf/icde	/conf/icde/icde
A	SIGIR	International ACM SIGIR Conference on Research and Development in Information Retrieval	/conf/sigir	/conf/sigir/sigir
A	VLDB	International Conference on Very Large Data Bases	/conf/vldb	/conf/vldb/vldb
A	VLDB	International Conference on Very Large Data Bases	/journals/pvldb	/journals/pvldb/pvldb
B	CIKM	ACM International Conference on Information and Knowledge Management	/conf/cikm	/conf/cikm/cikm
B	WSDM	ACM International Conference on Web Search and Data Mining	/conf/wsdm	/conf/wsdm/wsdm
B	PODS	ACM SIGMOD-SIGACT-SIGAI Symposium on Principles of Database Systems	/conf/pods	/conf/pods/pods
B	DASFAA	International Conference on Database Systems for Advanced Applications	/conf/dasfaa	/conf/dasfaa/dasfaa
B	ECML-PKDD	European Conference on Machine Learning and Principles and Practice of Knowledge Discovery in Databases	/conf/ecml	/conf/ecml/ecml
B	ECML-PKDD	European Conference on Machine Learning and Principles and Practice of Knowledge Discovery in Databases	/conf/pkdd	/conf/pkdd/pkdd
B	ISWC	IEEE International Semantic Web Conference	/conf/semweb	/conf/semweb/iswc
B	ICDM	IEEE International Conference on Data Mining	/conf/icdm	/conf/icdm/icdm
B	ICDT	International Conference on Database Theory	/conf/icdt	/conf/icdt/icdt
B	EDBT	International Conference on Extending Database Technology	/conf/edbt	/conf/edbt/edbt
B	CIDR	Conference on Innovative Data Systems Research	/conf/cidr	/conf/cidr/cidr
B	SDM	SIAM International Conference on Data Mining	/conf/sdm	/conf/sdm/sdm
B	RecSys	ACM Conference on Recommender Systems	/conf/recsys	/conf/recsys/recsys
B	WISE	Web Information Systems Engineering Conference	/conf/wise	/conf/wise/wise
C	APWeb	Asia Pacific Web Conference	/conf/apweb	/conf/apweb/apweb
C	DEXA	International Conference on Database and Expert System Applications	/conf/dexa	/conf/dexa/dexa
C	ECIR	European Conference on Information Retrieval	/conf/ecir	/conf/ecir/ecir
C	ESWC	Extended Semantic Web Conference	/conf/esws	/conf/esws/eswc
C	WebDB	International Workshop on Web and Databases	/conf/webdb	/conf/webdb/webdb
C	ER	International Conference on Conceptual Modeling	/conf/er	/conf/er/er
C	MDM	International Conference on Mobile Data Management	/conf/mdm	/conf/mdm/mdm
C	SSDBM	International Conference on Scientific and Statistical Database Management	/conf/ssdbm	/conf/ssdbm/ssdbm
C	WAIM	International Conference on Web Age Information Management	/conf/waim	/conf/waim/waim
C	SSTD	International Symposium on Spatial and Temporal Databases	/conf/ssd	/conf/ssd/ssd
C	SSTD	International Symposium on Spatial and Temporal Databases	/conf/ssd	/conf/ssd/sstd
C	PAKDD	Pacific-Asia Conference on Knowledge Discovery and Data Mining	/conf/pakdd	/conf/pakdd/pakdd
C	ADMA	International Conference on Advanced Data Mining and Applications	/conf/adma	/conf/adma/adma
C	WISA	Web Information Systems and Applications	/conf/wisa	/conf/wisa/wisa
A	TIT	IEEE Transactions on Information Theory	/journals/tit	/journals/tit/tit
A	IANDC	Information and Computation	/journals/iandc	/journals/iandc/iandc
A	SICOMP	SIAM Journal on Computing	/journals/siamcomp	/journals/siamcomp/siamcomp
B	TALG	ACM Transactions on Algorithms	/journals/talg	/journals/talg/talg
B	TOCL	ACM Transactions on Computational Logic	/journals/tocl	/journals/tocl/tocl
B	TOMS	ACM Transactions on Mathematical Software	/journals/toms	/journals/toms/toms
B	Algorithmica	Algorithmica	/journals/algorithmica	/journals/algorithmica/algorithmica
B	CC	Computational Complexity	/journals/cc	/journals/cc/cc
B	FAC	Formal Aspects of Computing	/journals/fac	/journals/fac/fac
B	FMSD	Formal Methods in System Design	/journals/fmsd	/journals/fmsd/fmsd
B	INFORMS	INFORMS Journal on Computing	/journals/informs	/journals/informs/informs
B	JCSS	Journal of Computer and System Sciences	/journals/jcss	/journals/jcss/jcss
B	JGO	Journal of Global Optimization	/journals/jgo	/journals/jgo/jgo
B	JSC	Journal of Symbolic Computation	/journals/jsc	/journals/jsc/jsc
B	MSCS	Mathematical Structures in Computer Science	/journals/mscs	/journals/mscs/mscs
B	TCS	Theoretical Computer Science	/journals/tcs	/journals/tcs/tcs
C	ACTA	Acta Informatica	/journals/acta	/journals/acta/acta
C	APAL	Annals of Pure and Applied Logic	/journals/apal	/journals/apal/apal
C	DAM	Discrete Applied Mathematics	/journals/dam	/journals/dam/dam
C	FUIN	Fundamenta Informaticae	/journals/fuin	/journals/fuin/fuin
C	IPL	Information Processing Letters	/journals/ipl	/journals/ipl/ipl
C	JCOMPLEXITY	Journal of Complexity	/journals/jc	/journals/jc/jc
C	LOGCOM	Journal of Logic and Computation	/journals/logcom	/journals/logcom/logcom
C	JSL	Journal of Symbolic Logic	/journals/jsyml	/journals/jsyml/jsyml
C	LMCS	Logical Methods in Computer Science	/journals/lmcs	/journals/lmcs/lmcs
C	SIDMA	SIAM Journal on Discrete Mathematics	/journals/siamdm	/journals/siamdm/siamdm
C		Theory of Computing Systems	/journals/mst	/journals/mst/mst
C	TQC	ACM Transactions in Quantum Computing	/journals/tqc	/journals/tqc/tqc
A	STOC	ACM Symposium on the Theory of Computing	/conf/stoc	/conf/stoc/stoc
A	SODA	ACM-SIAM Symposium on Discrete Algorithms	/conf/soda	/conf/soda/soda
A	CAV	International Conference on Computer Aided Verification	/conf/cav	/conf/cav/cav
A	FOCS	IEEE Annual Symposium on Foundations of Computer Science	/conf/focs	/conf/focs/focs
A	LICS	ACM/IEEE Symposium on Logic in Computer Science	/conf/lics	/conf/lics/lics
B	SoCG	International Symposium on Computational Geometry	/conf/compgeom	/conf/compgeom/compgeom
B	ESA	European Symposium on Algorithms	/conf/esa	/conf/esa/esa
B	CCC	Conference on Computational Complexity	/conf/coco	/conf/coco/coco
B	ICALP	International Colloquium on Automata, Languages and Programming	/conf/icalp	/conf/icalp/icalp
B	CADE	Conference on Automated Deduction	/conf/cade	/conf/cade/ijcar
B	CADE	Conference on Automated Deduction	/conf/cade	/conf/cade/cade
B	CONCUR	International Conference on Concurrency Theory	/conf/concur	/conf/concur/concur
B	HSCC	International Conference on Hybrid Systems: Computation and Control	/conf/hybrid	/conf/hybrid/hscc
B	SAT	International Conference on Theory and Applications of Satisfiability Testing	/conf/sat	/conf/sat/sat
B	COCOON	International Computing and Combinatorics Conference	/conf/cocoon	/conf/cocoon/cocoon
B	FMCAD	Formal Methods in Computer-Aided Design	/conf/fmcad	/conf/fmcad/fmcad
C	CSL	Computer Science Logic	/conf/csl	/conf/csl/csl
C	FSTTCS	Foundations of Software Technology and Theoretical Computer Science	/conf/fsttcs	/conf/fsttcs/fsttcs
C	DSAA	IEEE International Conference on Data Science and Advanced Analytics	/conf/dsaa	/conf/dsaa/dsaa
C	ICTAC	International Colloquium on Theoretical Aspects of Computing	/conf/ictac	/conf/ictac/ictac
C	IPCO	International Conference on Integer Programming and Combinatorial Optimization	/conf/ipco	/conf/ipco/ipco
C	FSCD	International Conference on Formal Structures for Computation and Deduction	/conf/rta	/conf/rta/rta
C	ISAAC	International Symposium on Algorithms and Computation	/conf/isaac	/conf/isaac/isaac
C	MFCS	Mathematical Foundations of Computer Science	/conf/mfcs	/conf/mfcs/mfcs
C	STACS	Symposium on Theoretical Aspects of Computer Science	/conf/stacs	/conf/stacs/stacs
C	SETTA	International Symposium on Software Engineering: Theories, Tools, and Applications	/conf/setta	/conf/setta/setta
A	TOG	ACM Transactions on Graphics	/journals/tog	/journals/tog/tog
A	TIP	IEEE Transactions on Image Processing	/journals/tip	/journals/tip/tip
A	TVCG	IEEE Transactions on Visualization and Computer Graphics	/journals/tvcg	/journals/tvcg/tvcg
A	TMM	IEEE Transactions on Multimedia	/journals/tmm	/journals/tmm/tmm
B	TOMM	ACM Transactions on Multimedia Computing, Communications and Applications	/journals/tomccap	/journals/tomccap/tomccap
B	CAGD	Computer Aided Geometric Design	/journals/cagd	/journals/cagd/cagd
B	CGF	Computer Graphics Forum	/journals/cgf	/journals/cgf/cgf
B	CAD	Computer-Aided Design	/journals/cad	/journals/cad/cad
B	TCSVT	IEEE Transactions on Circuits and Systems for Video Technology	/journals/tcsv	/journals/tcsv/tcsv
B	JASA	Journal of The Acoustical Society of America		
B	SIIMS	SIAM Journal on Imaging Sciences	/journals/siamis	/journals/siamis/siamis
B	SPEECH	Speech Communication	/journals/speech	/journals/speech/speech
B	CVMJ	Computational Visual Media	/journals/cvm	/journals/cvm/cvm
C	CGTA	Computational Geometry: Theory and Applications	/journals/comgeo	/journals/comgeo/comgeo
C	CAVW	Computer animation & virtual worlds	/journals/jvca	/journals/jvca/jvca
C	C&G	Computers & Graphics	/journals/cg	/journals/cg/cg
C	DCG	Discrete & Computational Geometry	/journals/dcg	/journals/dcg/dcg
C	SPL	IEEE Signal Processing Letters	/journals/spl	/journals/spl/spl
C	IET-IPR	IET Image Processing	/journals/iet-ipr	/journals/iet-ipr/iet-ipr
C	JVCIR	Journal of Visual Communication and Image Representation	/journals/jvcir	/journals/jvcir/jvcir
C	MS	Multimedia Systems	/journals/mms	/journals/mms/mms
C	MTA	Multimedia Tools and Applications	/journals/mta	/journals/mta/mta
C	SIGPRO	Signal Processing	/journals/sigpro	/journals/sigpro/sigpro
C	SPIC	Signal Processing: Image Communication	/journals/spic	/journals/spic/spic
C	TVC	The Visual Computer	/journals/vc	/journals/vc/vc
C	VI	Visual Informatics	/journals/vi	/journals/vi/vi
C	VRIH	Virtual Reality & Intelligent Hardware	/journals/vrih	/journals/vrih/vrih
C	GMOD	Graphical Models	/journals/cvgip	/journals/cvgip/cvgip
A	ACM MM	ACM International Conference on Multimedia	/conf/mm	/conf/mm/mm
A	SIGGRAPH	ACM Special Interest Group on Computer Graphics	/conf/siggraph	/conf/siggraph/siggraph
A	VR	IEEE Conference on Virtual Reality and 3D User Interfaces	/conf/vr	/conf/vr/vr
A	IEEE VIS	IEEE Visualization Conference	/conf/visualization	/conf/visualization/visualization
B	ICMR	ACM SIGMM International Conference on Multimedia Retrieval	/conf/mir	/conf/mir/icmr
B	ICMR	ACM SIGMM International Conference on Multimedia Retrieval	/conf/mir	/conf/mir/mir
B	I3D	ACM SIGGRAPH Symposium on Interactive 3D Graphics and Games	/conf/si3d	/conf/si3d/si3d
B	SCA	ACM SIGGRAPH/Eurographics Symposium on Computer Animation	/conf/sca	/conf/sca/sca
B	DCC	Data Compression Conference	/conf/dcc	/conf/dcc/dcc
B	Eurographics	Annual Conference of the European Association for Computer Graphics	/conf/eurographics	/conf/eurographics/eg
B	EuroVis	Eurographics Conference on Visualization	/conf/vissym	/conf/vissym/vissym
B	EuroVis	Eurographics Conference on Visualization	/conf/vissym	/conf/vissym/eurovis
B	SGP	Eurographics Symposium on Geometry Processing	/conf/sgp	/conf/sgp/sgp
B	EGSR	Eurographics Symposium on Rendering	/conf/rt	/conf/rt/dl
B	EGSR	Eurographics Symposium on Rendering	/conf/rt	/conf/rt/eii
B	ICASSP	IEEE International Conference on Acoustics, Speech and Signal Processing	/conf/icassp	/conf/icassp/icassp
B	ICME	IEEE International Conference on Multimedia & Expo	/conf/icmcs	/conf/icmcs/icme
B	ISMAR	International Symposium on Mixed and Augmented Reality	/conf/ismar	/conf/ismar/ismar
B	PG	Pacific Conference onComputer Graphics and Applications	/conf/pg	/conf/pg/pg
B	SPM	Symposium on Solid and Physical Modeling	/conf/sma	/conf/sma/spm
B	SPM	Symposium on Solid and Physical Modeling	/conf/sma	/conf/sma/sma
B	INTER-SPEECH	Conference of the International Speech Communication Association	/conf/interspeech	/conf/interspeech/interspeech
C	VRST	ACM Symposium on Virtual Reality Software and Technology	/conf/vrst	/conf/vrst/vrst
C	CASAXR	International Conference on Computer Animation, Social Agents, and Extended Reality	/conf/ca	/conf/ca/casa
C	CGI	Computer Graphics International	/conf/cgi	/conf/cgi/cgi
C	GMP	Geometric Modeling and Processing	/conf/gmp	/conf/gmp/gmp
C	PacificVis	IEEE Pacific Visualization Symposium	/conf/apvis	/conf/apvis/pacificvis
C	PacificVis	IEEE Pacific Visualization Symposium	/conf/apvis	/conf/apvis/apvis
C	3DV	International Conference on 3D Vision	/conf/3dim	/conf/3dim/3dim
C	CAD/Graphics	International Conference on Computer-Aided Design and Computer Graphics	/conf/cadgraphics	/conf/cadgraphics/cadgraphics
C	ICIP	International Conference on Image Processing	/conf/icip	/conf/icip/icip
C	MMM	International Conference on Multimedia Modeling	/conf/mmm	/conf/mmm/mmm
C	MMAsia	ACM Multimedia Asia	/conf/mmasia	/conf/mmasia/mmasia
C	SMI	Shape Modeling International	/conf/smi	/conf/smi/smi
C	CVM	Computational Visual Media	/conf/cvm	/conf/cvm/cvm
C	PRCV	Chinese Conference on Pattern Recognition and Computer Vision	/conf/prcv	/conf/prcv/prcv
C	ICIG	International Conference on Image and Graphics	/conf/icig	/conf/icig/icig
C	NCMMSC	National Conference on Man-Machine Speech Communication	/conf/ncmmsc	/conf/ncmmsc/ncmmsc
C	ASRU	Automatic Speech Recognition and Understanding Workshop	/conf/asru	/conf/asru/asru
C	SLT	Spoken Language Technology	/conf/slt	/conf/slt/slt
A	AI	Artificial Intelligence	/journals/ai	/journals/ai/ai
A	TPAMI	IEEE Transactions on Pattern Analysis and Machine Intelligence	/journals/pami	/journals/pami/pami
A	IJCV	International Journal of Computer Vision	/journals/ijcv	/journals/ijcv/ijcv
A	JMLR	Journal of Machine Learning Research	/journals/jmlr	/journals/jmlr/jmlr
B	TAP	ACM Transactions on Applied Perception	/journals/tap	/journals/tap/tap
B	AAMAS	Autonomous Agents and Multi-Agent Systems	/journals/aamas	/journals/aamas/aamas
B		Computational Linguistics	/journals/coling	/journals/coling/coling
B	CVIU	Computer Vision and Image Understanding	/journals/cviu	/journals/cviu/cviu
B	DKE	Data and Knowledge Engineering	/journals/dke	/journals/dke/dke
B		Evolutionary Computation	/journals/ec	/journals/ec/ec
B	TAC	IEEE Transactions on Affective Computing	/journals/taffco	/journals/taffco/taffco
B	TASLP	IEEE Transactions on Audio, Speech and Language Processing	/journals/taslp	/journals/taslp/taslp
B		IEEE Transactions on Cybernetics	/journals/tcyb	/journals/tcyb/tcyb
B		IEEE Transactions on Cybernetics	/journals/tcyb	/journals/tsmc/tsmcb
B	TEC	IEEE Transactions on Evolutionary Computation	/journals/tec	/journals/tec/tec
B	TFS	IEEE Transactions on Fuzzy Systems	/journals/tfs	/journals/tfs/tfs
B	TNNLS	IEEE Transactions on Neural Networks and learning systems	/journals/tnn	/journals/tnn/tnn
B	IJAR	International Journal of Approximate Reasoning	/journals/ijar	/journals/ijar/ijar
B	JAIR	Journal of Artificial Intelligence Research	/journals/jair	/journals/jair/jair
B		Journal of Automated Reasoning	/journals/jar	/journals/jar/jar
B	JSLHR	Journal of Speech, Language, and Hearing Research		
B		Machine Learning	/journals/ml	/journals/ml/ml
B		Neural Computation	/journals/neco	/journals/neco/neco
B		Neural Networks	/journals/nn	/journals/nn/nn
B	PR	Pattern Recognition	/journals/pr	/journals/pr/pr
B	TACL	Transactions of the Association for Computational Linguistics	/journals/tacl	/journals/tacl/tacl
C	TALLIP	ACM Transactions on Asian and Low-Resource Language Information Processing	/journals/talip	/journals/talip/talip
C		Applied Intelligence	/journals/apin	/journals/apin/apin
C	AIM	Artificial Intelligence in Medicine	/journals/artmed	/journals/artmed/artmed
C		Artificial Life	/journals/alife	/journals/alife/alife
C		Computational Intelligence	/journals/ci	/journals/ci/ci
C		Computer Speech & Language	/journals/csl	/journals/csl/csl
C		Connection Science	/journals/connection	/journals/connection/connection
C	DSS	Decision Support Systems	/journals/dss	/journals/dss/dss
C	EAAI	Engineering Applications of Artificial Intelligence	/journals/eaai	/journals/eaai/eaai
C		Expert Systems	/journals/es	/journals/es/es
C	ESWA	Expert Systems with Applications	/journals/eswa	/journals/eswa/eswa
C		Fuzzy Sets and Systems	/journals/fss	/journals/fss/fss
C	TG	IEEE Transactions on Games	/journals/tciaig	/journals/tciaig/tciaig
C	IET-CVI	IET Computer Vision	/journals/iet-cvi	/journals/iet-cvi/iet-cvi
C		IET Signal Processing	/journals/iet-spr	/journals/iet-spr/iet-spr
C	IVC	Image and Vision Computing	/journals/ivc	/journals/ivc/ivc
C	IDA	Intelligent Data Analysis	/journals/ida	/journals/ida/ida
C	IJCIA	International Journal of Computational Intelligence and Applications	/journals/ijcia	/journals/ijcia/ijcia
C	IJIS	International Journal of Intelligent Systems	/journals/ijis	/journals/ijis/ijis
C	IJNS	International Journal of Neural Systems	/journals/ijns	/journals/ijns/ijns
C	IJPRAI	International Journal of Pattern Recognition and Artificial Intelligence	/journals/ijprai	/journals/ijprai/ijprai
C	IJUFKS	International Journal of Uncertainty, Fuzziness and Knowledge-Based Systems	/journals/ijufks	/journals/ijufks/ijufks
C	IJDAR	International Journal on Document Analysis and Recognition	/journals/ijdar	/journals/ijdar/ijdar
C	JETAI	Journal of Experimental and Theoretical Artificial Intelligence	/journals/jetai	/journals/jetai/jetai
C	KBS	Knowledge-Based Systems	/journals/kbs	/journals/kbs/kbs
C		Machine Translation	/journals/mt	/journals/mt/mt
C		Machine Vision and Applications	/journals/mva	/journals/mva/mva
C		Natural Computing	/journals/nc	/journals/nc/nc
C	NLE	Natural Language Engineering	/journals/nle	/journals/nle/nle
C	NCA	Neural Computing and Applications	/journals/nca	/journals/nca/nca
C	NPL	Neural Processing Letters	/journals/npl	/journals/npl/npl
C		Neurocomputing	/journals/ijon	/journals/ijon/ijon
C	PAA	Pattern Analysis and Applications	/journals/paa	/journals/paa/paa
C	PRL	Pattern Recognition Letters	/journals/prl	/journals/prl/prl
C		Soft Computing	/journals/soco	/journals/soco/soco
C	WI	Web Intelligence	/journals/wias	/journals/wias/wias
C	TIIS	ACM Transactions on Interactive Intelligent Systems	/journals/tiis	/journals/tiis/tiis
C	TELO	ACM Transactions on Evolutionary Learning and Optimization	/journals/telo	/journals/telo/telo
C	JATS	ACM Journal on Autonomous Transportation Systems		
A	AAAI	AAAI Conference on Artificial Intelligence	/conf/aaai	/conf/aaai/aaai
A	NeurIPS	Conference on Neural Information Processing Systems	/conf/nips	/conf/nips/neurips
A	NeurIPS	Conference on Neural Information Processing Systems	/conf/nips	/conf/nips/nips
A	ACL	Annual Meeting of the Association for Computational Linguistics	/conf/acl	/conf/acl/acl
A	CVPR	IEEE/CVF Computer Vision and Pattern Recognition Conference	/conf/cvpr	/conf/cvpr/cvpr
A	ICCV	International Conference on Computer Vision	/conf/iccv	/conf/iccv/iccv
A	ICML	International Conference on Machine Learning	/conf/icml	/conf/icml/icml
A	ICLR	International Conference on Learning Representations	/conf/iclr	/conf/iclr/iclr
B	COLT	Annual Conference on Computational Learning Theory	/conf/colt	/conf/colt/colt
B	EMNLP	Conference on Empirical Methods in Natural Language Processing	/conf/emnlp	/conf/emnlp/emnlp
B	ECAI	European Conference on Artificial Intelligence	/conf/ecai	/conf/ecai/ecai
B	ECCV	European Conference on Computer Vision	/conf/eccv	/conf/eccv/eccv
B	ICRA	IEEE International Conference on Robotics and Automation	/conf/icra	/conf/icra/icra
B	ICAPS	International Conference on Automated Planning and Scheduling	/conf/aips	/conf/aips/icaps
B	ICCBR	International Conference on Case-Based Reasoning	/conf/iccbr	/conf/iccbr/iccbr
B	COLING	International Conference on Computational Linguistics	/conf/coling	/conf/coling/coling
B	KR	International Conference on Principles of Knowledge Representation and Reasoning	/conf/kr	/conf/kr/kr
B	UAI	International Conference on Uncertainty in Artificial Intelligence	/conf/uai	/conf/uai/uai
B	AAMAS	International Joint Conference on Autonomous Agents and Multi-agent Systems	/conf/atal	/conf/atal/aamas
B	AAMAS	International Joint Conference on Autonomous Agents and Multi-agent Systems	/conf/ifaamas	/conf/ifaamas/aamas
B	PPSN	Parallel Problem Solving from Nature	/conf/ppsn	/conf/ppsn/ppsn
B	NAACL	North American Chapter of the Association for Computational Linguistics	/conf/naacl	/conf/naacl/naacl
B	IJCAI	International Joint Conference on Artificial Intelligence	/conf/ijcai	/conf/ijcai/ijcai
C	AISTATS	International Conference on Artificial Intelligence and Statistics	/conf/aistats	/conf/aistats/aistats
C	ACCV	Asian Conference on Computer Vision	/conf/accv	/conf/accv/accv
C	ACML	Asian Conference on Machine Learning	/conf/acml	/conf/acml/acml
C	BMVC	British Machine Vision Conference	/conf/bmvc	/conf/bmvc/bmvc
C	NLPCC	CCF International Conference on Natural Language Processing and Chinese Computing	/conf/nlpcc	/conf/nlpcc/nlpcc
C	CoNLL	Conference on Computational Natural Language Learning	/conf/conll	/conf/conll/conll
C	GECCO	Genetic and Evolutionary Computation Conference	/conf/gecco	/conf/gecco/gecco
C	ICTAI	IEEE International Conference on Tools with Artificial Intelligence	/conf/ictai	/conf/ictai/ictai
C	IROS	IEEE/RSJ International Conference on Intelligent Robots and Systems	/conf/iros	/conf/iros/iros
C	ALT	International Conference on Algorithmic Learning Theory	/conf/alt	/conf/alt/alt
C	ICANN	International Conference on Artificial Neural Networks	/conf/icann	/conf/icann/icann
C	FG	International Conference on Automatic Face and Gesture Recognition	/conf/fgr	/conf/fgr/fg
C	ICDAR	International Conference on Document Analysis and Recognition	/conf/icdar	/conf/icdar/icdar
C	ILP	International Conference on Inductive Logic Programming	/conf/ilp	/conf/ilp/ilp
C	KSEM	International conference on Knowledge Science,Engineering and Management	/conf/ksem	/conf/ksem/ksem
C	ICONIP	International Conference on Neural Information Processing	/conf/iconip	/conf/iconip/iconip
C	ICPR	International Conference on Pattern Recognition	/conf/icpr	/conf/icpr/icpr
C	IJCB	International Joint Conference on Biometrics	/conf/icb	/conf/icb/icb
C	IJCNN	International Joint Conference on Neural Networks	/conf/ijcnn	/conf/ijcnn/ijcnn
C	PRICAI	Pacific Rim International Conference on Artificial Intelligence	/conf/pricai	/conf/pricai/pricai
C	IEEE CEC	Congress on Evolutionary Computation	/conf/cec	/conf/cec/cec
C	DAI	International Conference on Distributed Artificial Intelligence	/conf/dai2	/conf/dai2/dai2
A	TOCHI	ACM Transactions on Computer-Human Interaction	/journals/tochi	/journals/tochi/tochi
A	IJHCS	International Journal of Human-Computer Studies	/journals/ijmms	/journals/ijmms/ijmms
B	CSCW	Computer Supported Cooperative Work	/journals/cscw	/journals/cscw/cscw
B	HCI	Human-Computer Interaction	/journals/hhci	/journals/hhci/hhci
B		IEEE Transactions on Human-Machine Systems	/journals/thms	/journals/thms/thms
B		IEEE Transactions on Human-Machine Systems	/journals/thms	/journals/tsmc/tsmcc
B	IWC	Interacting with Computers	/journals/iwc	/journals/iwc/iwc
B	IJHCI	International Journal of Human-Computer Interaction	/journals/ijhci	/journals/ijhci/ijhci
B	UMUAI	User Modeling and User-Adapted Interaction	/journals/umuai	/journals/umuai/umuai
B	TSMC	IEEE Transactions on Systems, Man, and Cybernetics: Systems	/journals/tsmc	/journals/tsmc/tsmc
B	CCF TPCI	CCF Transactions on Pervasive Computing and Interaction	/journals/ccftpci	/journals/ccftpci/ccftpci
C	BIT	Behaviour & Information Technology	/journals/behaviourIT	/journals/behaviourIT/behaviourIT
C	PUC	Personal and Ubiquitous Computing	/journals/puc	/journals/puc/puc
C	PMC	Pervasive and Mobile Computing	/journals/percom	/journals/percom/percom
C	PACMHCI	Proceedings of the ACM on Human-Computer Interaction	/journals/pacmhci	/journals/pacmhci/pacmhci
C	THRI	ACM Transactions on Human-Robot Interaction	/journals/thri	/journals/thri/thri
A	CSCW	ACM Conference On Computer-Supported Cooperative Work And Social Computing	/conf/cscw	/conf/cscw/cscw
A	CHI	ACM Conference on Human Factors in Computing Systems	/conf/chi	/conf/chi/chi
A	UbiComp	ACM international joint conference on Pervasive and Ubiquitous Computing	/conf/huc	/conf/huc/ubicomp
A	UIST	ACM Symposium on User Interface Software and Technology	/conf/uist	/conf/uist/uist
B	GROUP	ACM International Conference on Supporting Group Work	/conf/group	/conf/group/group
B	IUI	ACM International Conference on Intelligent User Interfaces	/conf/iui	/conf/iui/iui
B	ISS	ACM International Conference on Interactive Tabletops and Surfaces	/conf/tabletop	/conf/tabletop/iss
B	ISS	ACM International Conference on Interactive Tabletops and Surfaces	/conf/tabletop	/conf/tabletop/its
B	ECSCW	European Conference on Computer Supported Cooperative Work	/conf/ecscw	/conf/ecscw/ecscw
B	PERCOM	IEEE International Conference on Pervasive Computing and Communications	/conf/percom	/conf/percom/percom
B	MobileHCI	ACM International Conference on Mobile Human-Computer Interaction	/conf/mhci	/conf/mhci/mhci
B	ICWSM	The International AAAI Conference on Web and Social Media	/conf/icwsm	/conf/icwsm/icwsm
C	DIS	ACM SIGCHI Conference on Designing Interactive Systems	/conf/ACMdis	/conf/ACMdis/ACMdis
C	ICMI	ACM International Conference on Multimodal Interaction	/conf/icmi	/conf/icmi/icmi
C	ASSETS	International ACM SIGACCESS Conference on Computers and Accessibility	/conf/assets	/conf/assets/assets
C	GI	Graphics Interface	/conf/graphicsinterface	/conf/graphicsinterface/graphicsinterface
C	UIC	IEEE International Conference on Ubiquitous Intelligence and Computing	/conf/uic	/conf/uic/uic
C		IEEE World Haptics Conference	/conf/haptics	/conf/haptics/haptics
C	INTERACT	International Conference on Human-Computer Interaction of International Federation for Information Processing	/conf/interact	/conf/interact/interact
C	IDC	ACM Interaction Design and Children	/conf/acmidc	/conf/acmidc/idc
C	CollaborateCom	International Conference on Collaborative Computing: Networking, Applications and Worksharing	/conf/colcom	/conf/colcom/colcom
C	CSCWD	International Conference on Computer Supported Cooperative Work in Design	/conf/cscwd	/conf/cscwd/cscwd
C	CoopIS	International Conference on Cooperative Information Systems	/conf/coopis	/conf/coopis/coopis
C	MobiQuitous	International Conference on Mobile and Ubiquitous Systems: Computing, Networking and Services	/conf/mobiquitous	/conf/mobiquitous/mobiquitous
C	AVI	International Working Conference on Advanced Visual Interfaces	/conf/avi	/conf/avi/avi
C	GPC	Conference on Green, Pervasive and Cloud Computing	/conf/gpc	/conf/gpc/gpc
C	ICXR	CCF International Conference on Extended Reality	/conf/icxr	/conf/icxr/icxr
A 	JACM	Journal of the ACM	/journals/jacm	/journals/jacm/jacm
A 	Proc. IEEE	Proceedings of the IEEE	/journals/pieee	/journals/pieee/pieee
A	SCIS	Science China Information Sciences	/journals/chinaf	/journals/chinaf/chinaf
A	Bioinformatics	Bioinformatics	/journals/bioinformatics	/journals/bioinformatics/bioinformatics
B		Briefings in Bioinformatics	/journals/bib	/journals/bib/bib
B	Cognition	Cognition		
B	TASAE	IEEE Transactions on Automation Science and Engineering	/journals/tase	/journals/tase/tase
B	TGARS	IEEE Transactions on Geoscience and Remote Sensing	/journals/tgrs	/journals/tgrs/tgrs
B	TITS	IEEE Transactions on Intelligent Transportation Systems	/journals/tits	/journals/tits/tits
B	TMI	IEEE Transactions on Medical Imaging	/journals/tmi	/journals/tmi/tmi
B	TR	IEEE Transactions on Robotics	/journals/trob	/journals/trob/trob
B	TCBB	IEEE/ACM Transactions on Computational Biology and Bioinformatics	/journals/tcbb	/journals/tcbb/tcbb
B	JCST	Journal of Computer Science and Technology	/journals/jcst	/journals/jcst/jcst
B	JAMIA	Journal of the American Medical Informatics Association	/journals/jamia	/journals/jamia/jamia
B		PLOS Computational Biology	/journals/ploscb	/journals/ploscb/ploscb
B		The Computer Journal	/journals/cj	/journals/cj/cj
B	WWW	The Web Conference	/journals/www	/journals/www/www
B	FCS	Frontiers of Computer Science	/journals/fcsc	/journals/fcsc/fcsc
B	BCRA	Blockchain: Research and Applications	/journals/bcra	/journals/bcra/bcra
C		BMC Bioinformatics	/journals/bmcbi	/journals/bmcbi/bmcbi
C		Cybernetics and Systems	/journals/cas	/journals/cas/cas
C		IEEE Geoscience and Remote Sensing Letters	/journals/lgrs	/journals/lgrs/lgrs
C	JBHI	IEEE Journal of Biomedical and Health Informatics	/journals/titb	/journals/titb/titb
C	TBD	IEEE Transactions on Big Data	/journals/tbd	/journals/tbd/tbd
C		IET Intelligent Transport Systems		
C	JBI	Journal of Biomedical Informatics	/journals/jbi	/journals/jbi/jbi
C		Medical Image Analysis	/journals/mia	/journals/mia/mia
C	TII	IEEE Transactions on Industrial Informatics	/journals/tii	/journals/tii/tii
C	TCPS	ACM Transactions on Cyber-Physical Systems	/journals/tcps	/journals/tcps/tcps
C	TOCE	ACM Transactions on Computing Education	/journals/jeric	/journals/jeric/toce
C	TOCE	ACM Transactions on Computing Education	/journals/jeric	/journals/jeric/jeric
C	EITEE	ENGINEERING Information Technology & Electronic Engineering	/journals/jzusc	/journals/jzusc/jzusc
C	TCSS	IEEE Transactions on Computational Social Systems	/journals/tcss	/journals/tcss/tcss
C		IEEE Transactions on Reliability	/journals/tr	/journals/tr/tr
C	HEALTH	ACM Transactions on Computing for Healthcare	/journals/health	/journals/health/health
C	ACM DLT	ACM Distributed Ledger Technologies: Research and Practice	/journals/distribledger	/journals/distribledger/distribledger
A	WWW	The Web Conference	/conf/www	/conf/www/www
A	RTSS	IEEE Real-Time Systems Symposium	/conf/rtss	/conf/rtss/rtss
B	CogSci	Annual Meeting of the Cognitive Science Society	/conf/cogsci	/conf/cogsci/cogsci
B	BIBM	IEEE International Conference on Bioinformatics and Biomedicine	/conf/bibm	/conf/bibm/bibm
B	EMSOFT	International Conference on Embedded Software IEEE/ACM/IFIP	/conf/emsoft	/conf/emsoft/emsoft
B	ISMB	International conference on Intelligent Systems for Molecular Biology	/journals/bioinformatics	/journals/bioinformatics/bioinformatics
B	ISMB	International conference on Intelligent Systems for Molecular Biology	/conf/ismb	/conf/ismb/ismb
B	RECOMB	Annual International Conference on Research in Computational Molecular Biology	/conf/recomb	/conf/recomb/recomb
B	MICCAI	International Conference on Medical Image Computing and Computer-Assisted Intervention	/conf/miccai	/conf/miccai/miccai
B	WINE	Conference on Web and Internet Economics	/conf/wine	/conf/wine/wine
C	AMIA	American Medical Informatics Association Annual Symposium	/conf/amia	/conf/amia/amia
C	APBC	Asia Pacific Bioinformatics Conference	/conf/apbc	/conf/apbc/apbc
C	IEEE BigData	IEEE International Conference on Big Data	/conf/bigdataconf	/conf/bigdataconf/bigdataconf
C	IEEE CLOUD	IEEE International Conference on Cloud Computing	/conf/IEEEcloud	/conf/IEEEcloud/IEEEcloud
C	SMC	IEEE International Conference on Systems, Man, and Cybernetics	/conf/smc	/conf/smc/smc
C	COSIT	International Conference on Spatial Information Theory	/conf/cosit	/conf/cosit/cosit
C	ISBRA	International Symposium on Bioinformatics Research and Applications	/conf/isbra	/conf/isbra/isbra
C	SAGT	International Symposium on Algorithmic Game Theory	/conf/sagt	/conf/sagt/sagt
C	SIGSPATIAL	ACM Special Interest Group on Spatial Information	/conf/gis	/conf/gis/gis
C	SIGSPATIAL	ACM Special Interest Group on Spatial Information	/journals/sigspatial	/journals/sigspatial/sigspatial
C	ICIC	International Conference on Intelligent Computing	/conf/icic	/conf/icic/icic
C	ICSS	International Conference on Service Science	/conf/service	/conf/service/service
C	AFT	Advances in Financial Technologies	/conf/aft	/conf/aft/aft
C	IJTCS-FAW	International Joint Conference on Theoretical Computer Science - Frontier of Algorithmic Wisdom	/conf/faw	/conf/faw/faw
P		arXiv	/journals/corr	/journals/corr/corr
'''

# ── Generation ──

result = {}
uri_to_info = {}

for line in ccf_rank_list.strip().split('\n'):
    line = line.strip()
    if not line:
        continue

    parts = line.split('\t')
    if len(parts) < 5:
        continue

    rank = parts[0].strip()
    abbr = parts[1].strip()
    full_name = parts[2].strip()
    dblp_uri = parts[3].strip()

    if not full_name:
        continue

    # venue name → rank + abbr
    result[full_name] = {
        "rank": rank,
        "abbr": abbr if abbr else "",
    }

    # DBLP URI → rank + venue + abbr
    if dblp_uri:
        uri_to_info[dblp_uri] = {
            "rank": rank,
            "venue": full_name,
            "abbr": abbr if abbr else "",
        }

with open('venue_to_rank.json', 'w', encoding='utf-8') as f:
    json.dump(result, f, ensure_ascii=False, indent=2)
print(f"Generated venue_to_rank.json with {len(result)} entries.")

with open('dblp_uri_to_rank.json', 'w', encoding='utf-8') as f:
    json.dump(uri_to_info, f, ensure_ascii=False, indent=2)
print(f"Generated dblp_uri_to_rank.json with {len(uri_to_info)} entries.")

