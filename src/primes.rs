#[rustfmt::skip]
pub(crate) static PRIME_INV_64: [u64; 256] = [
    
           0xaaaaaaaaaaaaaaab ,   0xcccccccccccccccd ,   0x6db6db6db6db6db7 ,   0x2e8ba2e8ba2e8ba3 ,
	   0x4ec4ec4ec4ec4ec5 ,   0xf0f0f0f0f0f0f0f1 ,   0x86bca1af286bca1b ,   0xd37a6f4de9bd37a7 ,
	   0x34f72c234f72c235 ,   0xef7bdef7bdef7bdf ,   0x14c1bacf914c1bad ,   0x8f9c18f9c18f9c19 ,
	   0x82fa0be82fa0be83 ,   0x51b3bea3677d46cf ,   0x21cfb2b78c13521d ,   0xcbeea4e1a08ad8f3 ,
	   0x4fbcda3ac10c9715 ,   0xf0b7672a07a44c6b ,   0x193d4bb7e327a977 ,   0x7e3f1f8fc7e3f1f9 ,
	   0x9b8b577e613716af ,   0xa3784a062b2e43db ,   0xf47e8fd1fa3f47e9 ,   0xa3a0fd5c5f02a3a1 ,
	   0x3a4c0a237c32b16d ,   0xdab7ec1dd3431b57 ,   0x77a04c8f8d28ac43 ,   0xa6c0964fda6c0965 ,
	   0x90fdbc090fdbc091 ,   0x7efdfbf7efdfbf7f ,    0x3e88cb3c9484e2b ,   0xe21a291c077975b9 ,
	   0x3aef6ca970586723 ,   0xdf5b0f768ce2cabd ,   0x6fe4dfc9bf937f27 ,   0x5b4fe5e92c0685b5 ,
	   0x1f693a1c451ab30b ,   0x8d07aa27db35a717 ,   0x882383b30d516325 ,   0xed6866f8d962ae7b ,
	   0x3454dca410f8ed9d ,   0x1d7ca632ee936f3f ,   0x70bf015390948f41 ,   0xc96bdb9d3d137e0d ,
	   0x2697cc8aef46c0f7 ,   0xc0e8f2a76e68575b ,   0x687763dfdb43bb1f ,   0x1b10ea929ba144cb ,
	   0x1d10c4c0478bbced ,   0x63fb9aeb1fdcd759 ,   0x64afaa4f437b2e0f ,   0xf010fef010fef011 ,
	   0x28cbfbeb9a020a33 ,   0xff00ff00ff00ff01 ,   0xd624fd1470e99cb7 ,   0x8fb3ddbd6205b5c5 ,
	   0xd57da36ca27acdef ,   0xee70c03b25e4463d ,   0xc5b1a6b80749cb29 ,   0x47768073c9b97113 ,
	   0x2591e94884ce32ad ,   0xf02806abc74be1fb ,   0x7ec3e8f3a7198487 ,   0x58550f8a39409d09 ,
	   0xec9e48ae6f71de15 ,   0x2ff3a018bfce8063 ,   0x7f9ec3fcf61fe7b1 ,   0x89f5abe570e046d3 ,
	   0xda971b23f1545af5 ,   0x79d5f00b9a7862a1 ,   0x4dba1df32a128a57 ,   0x87530217b7747d8f ,
	   0x30baae53bb5e06dd ,   0xee70206c12e9b5b3 ,   0xcdde9462ec9dbe7f ,   0xafb64b05ec41cf4d ,
	    0x2944ff5aec02945 ,   0x2cb033128382df71 ,   0x1ccacc0c84b1c2a9 ,   0x19a93db575eb3a0b ,
	   0xcebeef94fa86fe2d ,   0x6faa77fb3f8df54f ,   0x68a58af00975a751 ,   0xd56e36d0c3efac07 ,
	   0xd8b44c47a8299b73 ,    0x2d9ccaf9ba70e41 ,    0x985e1c023d9e879 ,   0x2a343316c494d305 ,
	   0x70cb7916ab67652f ,   0xd398f132fb10fe5b ,   0x6f2a38a6bf54fa1f ,   0x211df689b98f81d7 ,
	    0xe994983e90f1ec3 ,   0xad671e44bed87f3b ,   0xf9623a0516e70fc7 ,   0x4b7129be9dece355 ,
	   0x190f3b7473f62c39 ,   0x63dacc9aad46f9a3 ,   0xc1108fda24e8d035 ,   0xb77578472319bd8b ,
	   0x473d20a1c7ed9da5 ,   0xfbe85af0fea2c8fb ,   0x58a1f7e6ce0f4c09 ,   0x1a00e58c544986f3 ,
	   0x7194a17f55a10dc1 ,   0x7084944785e33763 ,   0xba10679bd84886b1 ,   0xebe9c6bb31260967 ,
	   0x97a3fe4bd1ff25e9 ,   0x6c6388395b84d99f ,   0x8c51da6a1335df6d ,   0x46f3234475d5add9 ,
	   0x905605ca3c619a43 ,   0xcee8dff304767747 ,   0xff99c27f00663d81 ,   0xacca407f671ddc2b ,
	   0xe71298bac1e12337 ,   0xfa1e94309cd09045 ,   0xbebccb8e91496b9b ,   0x312fa30cc7d7b8bd ,
	   0x6160ff9e9f006161 ,   0x6b03673b5e28152d ,   0xfe802ffa00bfe803 ,   0xe66fe25c9e907c7b ,
	   0x3f8b236c76528895 ,   0xf6f923bf01ce2c0d ,   0x6c3d3d98bed7c42f ,   0x30981efcd4b010e7 ,
	   0x6f691fc81ebbe575 ,   0xb10480ddb47b52cb ,   0x74cd59ed64f3f0d7 ,    0x105cb81316d6c0f ,
	   0x9be64c6d91c1195d ,   0x71b3f945a27b1f49 ,   0x77d80d50e508fd01 ,   0xa5eb778e133551cd ,
	   0x18657d3c2d8a3f1b ,   0x2e40e220c34ad735 ,   0xa76593c70a714919 ,   0x1eef452124eea383 ,
	   0x38206dc242ba771d ,   0x4cd4c35807772287 ,   0x83de917d5e69ddf3 ,   0x882ef0403b4a6c15 ,
	   0xf8fb6c51c606b677 ,   0xb4abaac446d3e1fd ,   0xa9f83bbe484a14e9 ,    0xbebbc0d1ce874d3 ,
	   0xbd418eaf0473189f ,   0x44e3af6f372b7e65 ,   0xc87fdace4f9e5d91 ,   0xec93479c446bd9bb ,
	   0xdac4d592e777c647 ,   0xa63ea8c8f61f0c23 ,   0xe476062ea5cbbb6f ,   0xdf68761c69daac27 ,
	   0xb813d737637aa061 ,   0xa3a77aac1fb15099 ,   0x17f0c3e0712c5825 ,   0xfd912a70ff30637b ,
	   0xfbb3b5dc01131289 ,   0x856d560a0f5acdf7 ,   0x96472f314d3f89e3 ,   0xa76f5c7ed2253531 ,
	   0x816eae7c7bf69fe7 ,   0xb6a2bea4cfb1781f ,   0xa3900c53318e81ed ,   0x60aa7f5d9f148d11 ,
	   0x6be8c0102c7a505d ,   0x8ff3f0ed28728f33 ,   0x680e0a87e5ec7155 ,   0xbbf70fa49fe829b7 ,
	   0xd69d1e7b6a50ca39 ,   0x1a1e0f46b6d26aef ,   0x7429f9a7a8251829 ,   0xd9c2219d1b863613 ,
	   0x91406c1820d077ad ,   0x521f4ec02e3d2b97 ,   0xbb8283b63dc8eba5 ,   0x431eda153229ebbf ,
	   0xaf0bf78d7e01686b ,   0xa9ced0742c086e8d ,   0xc26458ad9f632df9 ,   0xbbff1255dff892af ,
	   0xcbd49a333f04d8fd ,   0xec84ed6f9cfdeff5 ,   0x97980cc40bda9d4b ,   0x777f34d524f5cbd9 ,
	   0x2797051d94cbbb7f ,   0xea769051b4f43b81 ,   0xce7910f3034d4323 ,   0x92791d1374f5b99b ,
	   0x89a5645cc68ea1b5 ,   0x5f8aacf796c0cf0b ,   0xf2e90a15e33edf99 ,   0x8e99e5feb897c451 ,
	   0xaca2eda38fb91695 ,   0x5d9b737be5ea8b41 ,   0x4aefe1db93fd7cf7 ,   0xa0994ef20b3f8805 ,
	   0x103890bda912822f ,   0xb441659d13a9147d ,   0x1e2134440c4c3f21 ,   0x263a27727a6883c3 ,
	   0x78e221472ab33855 ,   0x95eac88e82e6faff ,   0xf66c258317be8dab ,    0x9ee202c7cb91939 ,
	   0x8d2fca1042a09ea3 ,   0x82779c856d8b8bf1 ,   0x3879361cba8a223d ,   0xf23f43639c3182a7 ,
	   0xa03868fc474bcd13 ,   0x651e78b8c5311a97 ,   0x8ffce639c00c6719 ,   0xf7b460754b0b61cf ,
	   0x7b03f3359b8e63b1 ,   0xa55c5326041eb667 ,   0x647f88ab896a76f5 ,   0x8fd971434a55a46d ,
	   0x9fbf969958046447 ,   0x9986feba69be3a81 ,   0xa668b3e6d053796f ,   0x97694e6589f4e09b ,
	   0x37890c00b7721dbd ,   0x5ac094a235f37ea9 ,   0x31cff775f2d5d65f ,   0xddad8e6b36505217 ,
	   0x5a27df897062cd03 ,   0xe2396fe0fdb5a625 ,   0xb352a4957e82317b ,   0xd8ab3f2c60c2ea3f ,
	   0x6893f702f0452479 ,   0x9686fdc182acf7e3 ,   0x6854037173dce12f ,   0x7f0ded1685c27331 ,
	   0xeeda72e1fe490b7d ,   0x9e7bfc959a8e6e53 ,   0x49b314d6d4753dd7 ,   0x2e8f8c5ac4aa1b3b ,
	   0xb8ef723481163d33 ,   0x6a2ec96a594287b7 ,   0xdba41c6d13aab8c5 ,   0xc2adbe648dc3aaf1 ,
	   0x87a2bade565f91a7 ,   0x4d6fe8798c01f5df ,   0x3791310c8c23d98b ,   0xf80e446b01228883 ,
	   0x9aed1436fbf500cf ,   0x7839b54cc8b24115 ,   0xc128c646ad0309c1 ,   0x14de631624a3c377 ,
	   0x3f7b9fe68b0ecbf9 ,   0x284ffd75ec00a285 ,   0x37803cb80dea2ddb ,   0x86b63f7c9ac4c6fd ,
	
];

#[rustfmt::skip]
pub(crate) static PRIME_INV_128: [u128; 128] = [
    
          0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab ,  0xcccccccccccccccccccccccccccccccd ,  
          0xb6db6db6db6db6db6db6db6db6db6db7 ,  0xa2e8ba2e8ba2e8ba2e8ba2e8ba2e8ba3 ,
	  0xc4ec4ec4ec4ec4ec4ec4ec4ec4ec4ec5 ,  0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f1 ,  
	  0xbca1af286bca1af286bca1af286bca1b ,  0x4de9bd37a6f4de9bd37a6f4de9bd37a7 ,
	  0xc234f72c234f72c234f72c234f72c235 ,  0xdef7bdef7bdef7bdef7bdef7bdef7bdf ,  
	  0xc1bacf914c1bacf914c1bacf914c1bad ,  0x18f9c18f9c18f9c18f9c18f9c18f9c19 ,
	  0xbe82fa0be82fa0be82fa0be82fa0be83 ,  0x3677d46cefa8d9df51b3bea3677d46cf ,  
	  0x13521cfb2b78c13521cfb2b78c13521d ,  0x8f2fba9386822b63cbeea4e1a08ad8f3 ,
	  0x14fbcda3ac10c9714fbcda3ac10c9715 ,  0xc2dd9ca81e9131abf0b7672a07a44c6b ,  
	  0x4f52edf8c9ea5dbf193d4bb7e327a977 ,  0x3f1f8fc7e3f1f8fc7e3f1f8fc7e3f1f9 ,
	  0xd5df984dc5abbf309b8b577e613716af ,  0x2818acb90f6bf3a9a3784a062b2e43db ,  
	  0xd1fa3f47e8fd1fa3f47e8fd1fa3f47e9 ,  0x5f02a3a0fd5c5f02a3a0fd5c5f02a3a1 ,
	  0xc32b16cfd7720f353a4c0a237c32b16d ,  0xd0c6d5bf60ee9a18dab7ec1dd3431b57 ,  
	  0xa2b10bf66e0e5aea77a04c8f8d28ac43 ,  0xc0964fda6c0964fda6c0964fda6c0965 ,
	  0xc090fdbc090fdbc090fdbc090fdbc091 ,  0xbf7efdfbf7efdfbf7efdfbf7efdfbf7f ,  
	  0xf82ee6986d6f63aa03e88cb3c9484e2b ,  0x21a291c077975b8fe21a291c077975b9 ,
	  0xa2126ad1f4f31ba03aef6ca970586723 ,  0x93c225cc74d50c06df5b0f768ce2cabd ,  
	  0x26fe4dfc9bf937f26fe4dfc9bf937f27 ,   0x685b4fe5e92c0685b4fe5e92c0685b5 ,
	  0x8bc775ca99ea03241f693a1c451ab30b ,  0x513ed9ad38b7f3bc8d07aa27db35a717 ,  
	  0x133caba736c05eb4882383b30d516325 ,   0xe4d3aa30a02dc3eed6866f8d962ae7b ,
	  0x6fbc1c498c05a84f3454dca410f8ed9d ,  0x7749b79f7f5470961d7ca632ee936f3f ,  
	  0x90948f40feac6f6b70bf015390948f41 ,   0xbb207cc0532ae21c96bdb9d3d137e0d ,
	  0x7a3607b7f5b5630e2697cc8aef46c0f7 ,  0x2f514a026d31be7bc0e8f2a76e68575b ,  
	  0xdd8f7f6d0eec7bfb687763dfdb43bb1f ,  0x766a024168e18cf81b10ea929ba144cb ,
	   0xc4c0478bbcecfee1d10c4c0478bbced ,  0x758fee6bac7f735d63fb9aeb1fdcd759 ,   
	   0x77f76e538c5167e64afaa4f437b2e0f ,  0x10fef010fef010fef010fef010fef011 ,
	  0xa020a32fefae680828cbfbeb9a020a33 ,  0xff00ff00ff00ff00ff00ff00ff00ff01 ,  
	  0xf836826ef73d52bcd624fd1470e99cb7 ,  0x3ce8354b2ea1c8cd8fb3ddbd6205b5c5 ,
	  0x8715ba188f963302d57da36ca27acdef ,  0xb25e4463cff13686ee70c03b25e4463d ,  
	  0x6c69ae01d272ca3fc5b1a6b80749cb29 ,  0xf26e5c44bfc61b2347768073c9b97113 ,
	  0xb07dd0d1b15d7cf12591e94884ce32ad ,  0xd2f87ebfcaa1c5a0f02806abc74be1fb ,  
	  0xbe25dd6d7aa646ca7ec3e8f3a7198487 ,  0xbc1d71afd8bdc03458550f8a39409d09 ,
	  0x2ed6d05a72acd1f7ec9e48ae6f71de15 ,  0x62ff3a018bfce8062ff3a018bfce8063 ,  
	  0x3fcf61fe7b0ff3d87f9ec3fcf61fe7b1 ,  0x398b6f668c2c43df89f5abe570e046d3 ,
	  0x8c1a682913ce1eceda971b23f1545af5 ,   0xb9a7862a0ff465879d5f00b9a7862a1 ,  
	  0xe7c13f77161b18f54dba1df32a128a57 ,  0x73186a06f9b8d9a287530217b7747d8f ,
	  0x7c39a6c708ec18b530baae53bb5e06dd ,  0x37634af9ebbc742dee70206c12e9b5b3 ,  
	  0x503578fb5236cf34cdde9462ec9dbe7f ,  0xbcdfc0d2975ccab1afb64b05ec41cf4d ,
	  0xf5aec02944ff5aec02944ff5aec02945 ,  0xc7d208f00a36e71a2cb033128382df71 ,  
	  0xd38f55c0280f05a21ccacc0c84b1c2a9 ,  0xca3be03aa7687a3219a93db575eb3a0b ,
	  0x6a69ce2344b66c3ccebeef94fa86fe2d ,  0xfecfe37d53bfd9fc6faa77fb3f8df54f ,  
	  0xa58af00975a750ff68a58af00975a751 ,  0xdc6da187df580dfed56e36d0c3efac07 ,
	  0x8fe44308ab0d4a8bd8b44c47a8299b73 ,  0xf1bf0091f5bcb8bb02d9ccaf9ba70e41 ,  
	  0x5e1c023d9e878ff70985e1c023d9e879 ,  0x7880d53da3d15a842a343316c494d305 ,
	  0x1ddb81ef699b5e8c70cb7916ab67652f ,  0xf364512170607acad398f132fb10fe5b ,  
	  0xadb1f8848af4c6d06f2a38a6bf54fa1f ,  0xd9a0541b55af0c17211df689b98f81d7 ,
	  0x673bf5928258a2ac0e994983e90f1ec3 ,   0xdda093c0628041aad671e44bed87f3b ,  
	  0xa9fcf24229bbcd1af9623a0516e70fc7 ,  0xcbb18a4f7732cc324b7129be9dece355 ,
	   0x1f727cce5f530a5190f3b7473f62c39 ,  0x6da4f4bdeb71121c63dacc9aad46f9a3 ,  
	  0x4d9abc552cf42b88c1108fda24e8d035 ,  0x141fd3124095c328b77578472319bd8b ,
	  0xddfd3e0bf3218d19473d20a1c7ed9da5 ,  0xdb2b3278f3b910d2fbe85af0fea2c8fb ,  
	  0xcb5c3b636e3a7d1358a1f7e6ce0f4c09 ,  0x1bcbfe34e7576cf21a00e58c544986f3 ,
	  0x6b5e80aa5ef23f007194a17f55a10dc1 ,  0x9a628feb11022e3a7084944785e33763 ,  
	  0xbe61909edde53c01ba10679bd84886b1 ,  0x4feb7c5e05fbb9e8ebe9c6bb31260967 ,
	  0x1ff25e8ff92f47fc97a3fe4bd1ff25e9 ,  0x30143e6b1fa187616c6388395b84d99f ,  
	  0xd49154c6c94ac0f08c51da6a1335df6d ,  0x9b9771454a44e00d46f3234475d5add9 ,
	  0x3aba1b4baef0b2a9905605ca3c619a43 ,  0xcc11d9dd1bfe608ecee8dff304767747 ,  
	  0xff99c27f00663d80ff99c27f00663d81 ,  0x111ea8032f60bf1aacca407f671ddc2b ,
	  0xdd9395f5b667aa88e71298bac1e12337 ,  0xa7caaed93038740afa1e94309cd09045 ,  
	  0x2be5958f582e9db7bebccb8e91496b9b ,  0x995e1ca8dbfb5a3d312fa30cc7d7b8bd ,
	  0x9f006160ff9e9f006160ff9e9f006161 ,  0xb33ce15ee9b097416b03673b5e28152d ,  
	  0xfa00bfe802ffa00bfe802ffa00bfe803 ,  0x1c2802f6bcf18d26e66fe25c9e907c7b ,
	  0xcf6dec4793e72aba3f8b236c76528895 ,  0x1e547da72d224d44f6f923bf01ce2c0d ,  
	  0x7746da9d5fc708306c3d3d98bed7c42f ,  0xcdff4bb55916e37a30981efcd4b010e7 ,
	  
	 
];