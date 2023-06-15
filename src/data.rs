
pub(crate) const K_PLUS : [u64;18] = [
  10534,10680,13086,19474,21430,21915,21945,29051,38811,43489,45124,49659,50544,50751,51150,57180,57311,58062
];

// Bases selected from miller-rabin appspot conjectured to be strong bases
pub(crate) const STRONG_BASE : [u64;33] =[
  9345883071009581737,336781006125, 9639812373923155,4230279247111683200, 14694767155120705706, 16641139526367750375,
  141889084524735, 1199124725622454117, 11096072698276303650,4130806001517, 149795463772692060, 186635894390467037, 3967304179347715805,
  23635709730000, 9233062284813009, 43835965440333360, 761179012939631437, 1263739024124850375,325, 9375, 28178, 450775, 9780504, 1795265022,
  126401071349994536,62769592775616394,34933608779780163,1948244569546278,1769236083487960,64390572806844,814494960528,921211727,377687
  
];
// First bases minus the perfect powers
pub(crate) const FIRST : [u64;90] = [
	2,3,5,6,7,10,11,13,14,15,17,18,19,20,21,22,23,24,26,28,29,30,31,
	32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,50,51,52,53,54,
	55,56,57,58,59,60,61,62,63,65,66,67,68,69,70,71,72,73,74,75,76,77,
	78,79,80,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,
	101
];

const PRIMORIAL : [u64;7] = [210,2310,30030,510510,9699690,223092870,6469693230];

// Randomly Generated 32-bit composites
const COMPOSITE : [u64;102] = [
	2152470778,2395843435,2568773150,2766100730,2896851094,3072015257,3191478047,3302442981,
	3486062840,3663157973,3883159676,4033414738,4177101034,2159636228,2402509207,2569168793,
	2769220655,2937455460,3089274691,3193413738,3359574670,3498807101,3691604479,3898497877,  
	4036579037,4221312674,2223095592,2403645433,2636884866,2773496999,2942261140,3089981073,
	3198446445,3378951452,3537135748,3694523079,3915115878,4045003933,4250246851,2240296935,
	2406407369,2662884868,2785242335,2947533447,3106622765,2458856404,3379446792,3577324273,
	3743751708,3939385071,4048980891,4251573755,2329602134,2411788689,2682498300,2795894007,
	2961406037,3122349870,3224481965,3381689731,3637688426,3743864483,3951672793,4087984065,
	2330783561,2494447392,2705267189,2833385729,2992427549,3145332659,3226144448,3401427309,
	3650580462,3800214167,3967966652,4098402874,2331603433,2511815268,2747138761,2850208345,  
	2996639544,3159770762,3255909586,3412417313,3656229161,3846593071,3981004793,4140745633,
	2363687496,2551278956,2753594093,2892357738,3002496030,3169227212,3276044686,3467526266,
	3657882624,3877905860,4021117111,4164884518,3687570078,2281720546
];

// Randomly generated 32-bit primes
const PRIME : [u64;102] = [
	2157797017,2264989201,2363812613,2476290247,2651049061,2817213457,2971354663,3121314323,3306967673,
	3422955973,3667630613,3928222471,4117033541,2167194413,2278925819,2365759793,2516545217,2670948799,  
	2824390111,3005333993,3139634383,3317753717,3436161877,3684799207,3947417741,4123695617,2192841559,  
	2289647981,2366021059,2555490313,2671067099,2843821027,3007771423,3184255681,3331554269,3449480063,  
	3745731299,3981323317,4136687791,2214869039,2310916459,2390299369,2560057771,2677389521,2846388487,  
	3047522377,3189286517,3336827239,3493803941,3807095003,3989516813,4163942803,2222708573,2325495323,  
	2402424917,2600020481,2693101897,2848658903,3066672299,3221727451,3351071011,3565469023,3823242763,  
	3994356001,4213449673,2243776879,2344957739,2428378213,2600378161,2784752567,2864970257,3067070329,  
	3234772771,3351833549,3585679927,3853509853,4012752281,4270765963,2250688051,2354804093,2448415379,
	2644208129,2795861177,2876296933,3093624713,3247469959,3381574549,3589120207,3897523607,4051083821,
	2257700527,2363438221,2466504059,2645561473,2808738571,2962273547,3104565733,3284423237,3382337437,
	3629744413,3903858349,4092284837
];
