// std
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};
// others
use num::Zero;
// pbrt
use core::pbrt::{Float, Spectrum};
use core::pbrt::{clamp_t, find_interval, lerp};

// see spectrum.h

pub const N_CIE_SAMPLES: u16 = 471_u16;
pub const CIE_X: [Float; N_CIE_SAMPLES as usize] = [
    // CIE X function values
    0.0001299000,
    0.0001458470,
    0.0001638021,
    0.0001840037,
    0.0002066902,
    0.0002321000,
    0.0002607280,
    0.0002930750,
    0.0003293880,
    0.0003699140,
    0.0004149000,
    0.0004641587,
    0.0005189860,
    0.0005818540,
    0.0006552347,
    0.0007416000,
    0.0008450296,
    0.0009645268,
    0.001094949,
    0.001231154,
    0.001368000,
    0.001502050,
    0.001642328,
    0.001802382,
    0.001995757,
    0.002236000,
    0.002535385,
    0.002892603,
    0.003300829,
    0.003753236,
    0.004243000,
    0.004762389,
    0.005330048,
    0.005978712,
    0.006741117,
    0.007650000,
    0.008751373,
    0.01002888,
    0.01142170,
    0.01286901,
    0.01431000,
    0.01570443,
    0.01714744,
    0.01878122,
    0.02074801,
    0.02319000,
    0.02620736,
    0.02978248,
    0.03388092,
    0.03846824,
    0.04351000,
    0.04899560,
    0.05502260,
    0.06171880,
    0.06921200,
    0.07763000,
    0.08695811,
    0.09717672,
    0.1084063,
    0.1207672,
    0.1343800,
    0.1493582,
    0.1653957,
    0.1819831,
    0.1986110,
    0.2147700,
    0.2301868,
    0.2448797,
    0.2587773,
    0.2718079,
    0.2839000,
    0.2949438,
    0.3048965,
    0.3137873,
    0.3216454,
    0.3285000,
    0.3343513,
    0.3392101,
    0.3431213,
    0.3461296,
    0.3482800,
    0.3495999,
    0.3501474,
    0.3500130,
    0.3492870,
    0.3480600,
    0.3463733,
    0.3442624,
    0.3418088,
    0.3390941,
    0.3362000,
    0.3331977,
    0.3300411,
    0.3266357,
    0.3228868,
    0.3187000,
    0.3140251,
    0.3088840,
    0.3032904,
    0.2972579,
    0.2908000,
    0.2839701,
    0.2767214,
    0.2689178,
    0.2604227,
    0.2511000,
    0.2408475,
    0.2298512,
    0.2184072,
    0.2068115,
    0.1953600,
    0.1842136,
    0.1733273,
    0.1626881,
    0.1522833,
    0.1421000,
    0.1321786,
    0.1225696,
    0.1132752,
    0.1042979,
    0.09564000,
    0.08729955,
    0.07930804,
    0.07171776,
    0.06458099,
    0.05795001,
    0.05186211,
    0.04628152,
    0.04115088,
    0.03641283,
    0.03201000,
    0.02791720,
    0.02414440,
    0.02068700,
    0.01754040,
    0.01470000,
    0.01216179,
    0.009919960,
    0.007967240,
    0.006296346,
    0.004900000,
    0.003777173,
    0.002945320,
    0.002424880,
    0.002236293,
    0.002400000,
    0.002925520,
    0.003836560,
    0.005174840,
    0.006982080,
    0.009300000,
    0.01214949,
    0.01553588,
    0.01947752,
    0.02399277,
    0.02910000,
    0.03481485,
    0.04112016,
    0.04798504,
    0.05537861,
    0.06327000,
    0.07163501,
    0.08046224,
    0.08973996,
    0.09945645,
    0.1096000,
    0.1201674,
    0.1311145,
    0.1423679,
    0.1538542,
    0.1655000,
    0.1772571,
    0.1891400,
    0.2011694,
    0.2133658,
    0.2257499,
    0.2383209,
    0.2510668,
    0.2639922,
    0.2771017,
    0.2904000,
    0.3038912,
    0.3175726,
    0.3314384,
    0.3454828,
    0.3597000,
    0.3740839,
    0.3886396,
    0.4033784,
    0.4183115,
    0.4334499,
    0.4487953,
    0.4643360,
    0.4800640,
    0.4959713,
    0.5120501,
    0.5282959,
    0.5446916,
    0.5612094,
    0.5778215,
    0.5945000,
    0.6112209,
    0.6279758,
    0.6447602,
    0.6615697,
    0.6784000,
    0.6952392,
    0.7120586,
    0.7288284,
    0.7455188,
    0.7621000,
    0.7785432,
    0.7948256,
    0.8109264,
    0.8268248,
    0.8425000,
    0.8579325,
    0.8730816,
    0.8878944,
    0.9023181,
    0.9163000,
    0.9297995,
    0.9427984,
    0.9552776,
    0.9672179,
    0.9786000,
    0.9893856,
    0.9995488,
    1.0090892,
    1.0180064,
    1.0263000,
    1.0339827,
    1.0409860,
    1.0471880,
    1.0524667,
    1.0567000,
    1.0597944,
    1.0617992,
    1.0628068,
    1.0629096,
    1.0622000,
    1.0607352,
    1.0584436,
    1.0552244,
    1.0509768,
    1.0456000,
    1.0390369,
    1.0313608,
    1.0226662,
    1.0130477,
    1.0026000,
    0.9913675,
    0.9793314,
    0.9664916,
    0.9528479,
    0.9384000,
    0.9231940,
    0.9072440,
    0.8905020,
    0.8729200,
    0.8544499,
    0.8350840,
    0.8149460,
    0.7941860,
    0.7729540,
    0.7514000,
    0.7295836,
    0.7075888,
    0.6856022,
    0.6638104,
    0.6424000,
    0.6215149,
    0.6011138,
    0.5811052,
    0.5613977,
    0.5419000,
    0.5225995,
    0.5035464,
    0.4847436,
    0.4661939,
    0.4479000,
    0.4298613,
    0.4120980,
    0.3946440,
    0.3775333,
    0.3608000,
    0.3444563,
    0.3285168,
    0.3130192,
    0.2980011,
    0.2835000,
    0.2695448,
    0.2561184,
    0.2431896,
    0.2307272,
    0.2187000,
    0.2070971,
    0.1959232,
    0.1851708,
    0.1748323,
    0.1649000,
    0.1553667,
    0.1462300,
    0.1374900,
    0.1291467,
    0.1212000,
    0.1136397,
    0.1064650,
    0.09969044,
    0.09333061,
    0.08740000,
    0.08190096,
    0.07680428,
    0.07207712,
    0.06768664,
    0.06360000,
    0.05980685,
    0.05628216,
    0.05297104,
    0.04981861,
    0.04677000,
    0.04378405,
    0.04087536,
    0.03807264,
    0.03540461,
    0.03290000,
    0.03056419,
    0.02838056,
    0.02634484,
    0.02445275,
    0.02270000,
    0.02108429,
    0.01959988,
    0.01823732,
    0.01698717,
    0.01584000,
    0.01479064,
    0.01383132,
    0.01294868,
    0.01212920,
    0.01135916,
    0.01062935,
    0.009938846,
    0.009288422,
    0.008678854,
    0.008110916,
    0.007582388,
    0.007088746,
    0.006627313,
    0.006195408,
    0.005790346,
    0.005409826,
    0.005052583,
    0.004717512,
    0.004403507,
    0.004109457,
    0.003833913,
    0.003575748,
    0.003334342,
    0.003109075,
    0.002899327,
    0.002704348,
    0.002523020,
    0.002354168,
    0.002196616,
    0.002049190,
    0.001910960,
    0.001781438,
    0.001660110,
    0.001546459,
    0.001439971,
    0.001340042,
    0.001246275,
    0.001158471,
    0.001076430,
    0.0009999493,
    0.0009287358,
    0.0008624332,
    0.0008007503,
    0.0007433960,
    0.0006900786,
    0.0006405156,
    0.0005945021,
    0.0005518646,
    0.0005124290,
    0.0004760213,
    0.0004424536,
    0.0004115117,
    0.0003829814,
    0.0003566491,
    0.0003323011,
    0.0003097586,
    0.0002888871,
    0.0002695394,
    0.0002515682,
    0.0002348261,
    0.0002191710,
    0.0002045258,
    0.0001908405,
    0.0001780654,
    0.0001661505,
    0.0001550236,
    0.0001446219,
    0.0001349098,
    0.0001258520,
    0.0001174130,
    0.0001095515,
    0.0001022245,
    0.00009539445,
    0.00008902390,
    0.00008307527,
    0.00007751269,
    0.00007231304,
    0.00006745778,
    0.00006292844,
    0.00005870652,
    0.00005477028,
    0.00005109918,
    0.00004767654,
    0.00004448567,
    0.00004150994,
    0.00003873324,
    0.00003614203,
    0.00003372352,
    0.00003146487,
    0.00002935326,
    0.00002737573,
    0.00002552433,
    0.00002379376,
    0.00002217870,
    0.00002067383,
    0.00001927226,
    0.00001796640,
    0.00001674991,
    0.00001561648,
    0.00001455977,
    0.00001357387,
    0.00001265436,
    0.00001179723,
    0.00001099844,
    0.00001025398,
    0.000009559646,
    0.000008912044,
    0.000008308358,
    0.000007745769,
    0.000007221456,
    0.000006732475,
    0.000006276423,
    0.000005851304,
    0.000005455118,
    0.000005085868,
    0.000004741466,
    0.000004420236,
    0.000004120783,
    0.000003841716,
    0.000003581652,
    0.000003339127,
    0.000003112949,
    0.000002902121,
    0.000002705645,
    0.000002522525,
    0.000002351726,
    0.000002192415,
    0.000002043902,
    0.000001905497,
    0.000001776509,
    0.000001656215,
    0.000001544022,
    0.000001439440,
    0.000001341977,
    0.000001251141,
];

pub const CIE_Y: [Float; N_CIE_SAMPLES as usize] = [
    // CIE Y function values
    0.000003917000,
    0.000004393581,
    0.000004929604,
    0.000005532136,
    0.000006208245,
    0.000006965000,
    0.000007813219,
    0.000008767336,
    0.000009839844,
    0.00001104323,
    0.00001239000,
    0.00001388641,
    0.00001555728,
    0.00001744296,
    0.00001958375,
    0.00002202000,
    0.00002483965,
    0.00002804126,
    0.00003153104,
    0.00003521521,
    0.00003900000,
    0.00004282640,
    0.00004691460,
    0.00005158960,
    0.00005717640,
    0.00006400000,
    0.00007234421,
    0.00008221224,
    0.00009350816,
    0.0001061361,
    0.0001200000,
    0.0001349840,
    0.0001514920,
    0.0001702080,
    0.0001918160,
    0.0002170000,
    0.0002469067,
    0.0002812400,
    0.0003185200,
    0.0003572667,
    0.0003960000,
    0.0004337147,
    0.0004730240,
    0.0005178760,
    0.0005722187,
    0.0006400000,
    0.0007245600,
    0.0008255000,
    0.0009411600,
    0.001069880,
    0.001210000,
    0.001362091,
    0.001530752,
    0.001720368,
    0.001935323,
    0.002180000,
    0.002454800,
    0.002764000,
    0.003117800,
    0.003526400,
    0.004000000,
    0.004546240,
    0.005159320,
    0.005829280,
    0.006546160,
    0.007300000,
    0.008086507,
    0.008908720,
    0.009767680,
    0.01066443,
    0.01160000,
    0.01257317,
    0.01358272,
    0.01462968,
    0.01571509,
    0.01684000,
    0.01800736,
    0.01921448,
    0.02045392,
    0.02171824,
    0.02300000,
    0.02429461,
    0.02561024,
    0.02695857,
    0.02835125,
    0.02980000,
    0.03131083,
    0.03288368,
    0.03452112,
    0.03622571,
    0.03800000,
    0.03984667,
    0.04176800,
    0.04376600,
    0.04584267,
    0.04800000,
    0.05024368,
    0.05257304,
    0.05498056,
    0.05745872,
    0.06000000,
    0.06260197,
    0.06527752,
    0.06804208,
    0.07091109,
    0.07390000,
    0.07701600,
    0.08026640,
    0.08366680,
    0.08723280,
    0.09098000,
    0.09491755,
    0.09904584,
    0.1033674,
    0.1078846,
    0.1126000,
    0.1175320,
    0.1226744,
    0.1279928,
    0.1334528,
    0.1390200,
    0.1446764,
    0.1504693,
    0.1564619,
    0.1627177,
    0.1693000,
    0.1762431,
    0.1835581,
    0.1912735,
    0.1994180,
    0.2080200,
    0.2171199,
    0.2267345,
    0.2368571,
    0.2474812,
    0.2586000,
    0.2701849,
    0.2822939,
    0.2950505,
    0.3085780,
    0.3230000,
    0.3384021,
    0.3546858,
    0.3716986,
    0.3892875,
    0.4073000,
    0.4256299,
    0.4443096,
    0.4633944,
    0.4829395,
    0.5030000,
    0.5235693,
    0.5445120,
    0.5656900,
    0.5869653,
    0.6082000,
    0.6293456,
    0.6503068,
    0.6708752,
    0.6908424,
    0.7100000,
    0.7281852,
    0.7454636,
    0.7619694,
    0.7778368,
    0.7932000,
    0.8081104,
    0.8224962,
    0.8363068,
    0.8494916,
    0.8620000,
    0.8738108,
    0.8849624,
    0.8954936,
    0.9054432,
    0.9148501,
    0.9237348,
    0.9320924,
    0.9399226,
    0.9472252,
    0.9540000,
    0.9602561,
    0.9660074,
    0.9712606,
    0.9760225,
    0.9803000,
    0.9840924,
    0.9874812,
    0.9903128,
    0.9928116,
    0.9949501,
    0.9967108,
    0.9980983,
    0.9991120,
    0.9997482,
    1.0000000,
    0.9998567,
    0.9993046,
    0.9983255,
    0.9968987,
    0.9950000,
    0.9926005,
    0.9897426,
    0.9864444,
    0.9827241,
    0.9786000,
    0.9740837,
    0.9691712,
    0.9638568,
    0.9581349,
    0.9520000,
    0.9454504,
    0.9384992,
    0.9311628,
    0.9234576,
    0.9154000,
    0.9070064,
    0.8982772,
    0.8892048,
    0.8797816,
    0.8700000,
    0.8598613,
    0.8493920,
    0.8386220,
    0.8275813,
    0.8163000,
    0.8047947,
    0.7930820,
    0.7811920,
    0.7691547,
    0.7570000,
    0.7447541,
    0.7324224,
    0.7200036,
    0.7074965,
    0.6949000,
    0.6822192,
    0.6694716,
    0.6566744,
    0.6438448,
    0.6310000,
    0.6181555,
    0.6053144,
    0.5924756,
    0.5796379,
    0.5668000,
    0.5539611,
    0.5411372,
    0.5283528,
    0.5156323,
    0.5030000,
    0.4904688,
    0.4780304,
    0.4656776,
    0.4534032,
    0.4412000,
    0.4290800,
    0.4170360,
    0.4050320,
    0.3930320,
    0.3810000,
    0.3689184,
    0.3568272,
    0.3447768,
    0.3328176,
    0.3210000,
    0.3093381,
    0.2978504,
    0.2865936,
    0.2756245,
    0.2650000,
    0.2547632,
    0.2448896,
    0.2353344,
    0.2260528,
    0.2170000,
    0.2081616,
    0.1995488,
    0.1911552,
    0.1829744,
    0.1750000,
    0.1672235,
    0.1596464,
    0.1522776,
    0.1451259,
    0.1382000,
    0.1315003,
    0.1250248,
    0.1187792,
    0.1127691,
    0.1070000,
    0.1014762,
    0.09618864,
    0.09112296,
    0.08626485,
    0.08160000,
    0.07712064,
    0.07282552,
    0.06871008,
    0.06476976,
    0.06100000,
    0.05739621,
    0.05395504,
    0.05067376,
    0.04754965,
    0.04458000,
    0.04175872,
    0.03908496,
    0.03656384,
    0.03420048,
    0.03200000,
    0.02996261,
    0.02807664,
    0.02632936,
    0.02470805,
    0.02320000,
    0.02180077,
    0.02050112,
    0.01928108,
    0.01812069,
    0.01700000,
    0.01590379,
    0.01483718,
    0.01381068,
    0.01283478,
    0.01192000,
    0.01106831,
    0.01027339,
    0.009533311,
    0.008846157,
    0.008210000,
    0.007623781,
    0.007085424,
    0.006591476,
    0.006138485,
    0.005723000,
    0.005343059,
    0.004995796,
    0.004676404,
    0.004380075,
    0.004102000,
    0.003838453,
    0.003589099,
    0.003354219,
    0.003134093,
    0.002929000,
    0.002738139,
    0.002559876,
    0.002393244,
    0.002237275,
    0.002091000,
    0.001953587,
    0.001824580,
    0.001703580,
    0.001590187,
    0.001484000,
    0.001384496,
    0.001291268,
    0.001204092,
    0.001122744,
    0.001047000,
    0.0009765896,
    0.0009111088,
    0.0008501332,
    0.0007932384,
    0.0007400000,
    0.0006900827,
    0.0006433100,
    0.0005994960,
    0.0005584547,
    0.0005200000,
    0.0004839136,
    0.0004500528,
    0.0004183452,
    0.0003887184,
    0.0003611000,
    0.0003353835,
    0.0003114404,
    0.0002891656,
    0.0002684539,
    0.0002492000,
    0.0002313019,
    0.0002146856,
    0.0001992884,
    0.0001850475,
    0.0001719000,
    0.0001597781,
    0.0001486044,
    0.0001383016,
    0.0001287925,
    0.0001200000,
    0.0001118595,
    0.0001043224,
    0.00009733560,
    0.00009084587,
    0.00008480000,
    0.00007914667,
    0.00007385800,
    0.00006891600,
    0.00006430267,
    0.00006000000,
    0.00005598187,
    0.00005222560,
    0.00004871840,
    0.00004544747,
    0.00004240000,
    0.00003956104,
    0.00003691512,
    0.00003444868,
    0.00003214816,
    0.00003000000,
    0.00002799125,
    0.00002611356,
    0.00002436024,
    0.00002272461,
    0.00002120000,
    0.00001977855,
    0.00001845285,
    0.00001721687,
    0.00001606459,
    0.00001499000,
    0.00001398728,
    0.00001305155,
    0.00001217818,
    0.00001136254,
    0.00001060000,
    0.000009885877,
    0.000009217304,
    0.000008592362,
    0.000008009133,
    0.000007465700,
    0.000006959567,
    0.000006487995,
    0.000006048699,
    0.000005639396,
    0.000005257800,
    0.000004901771,
    0.000004569720,
    0.000004260194,
    0.000003971739,
    0.000003702900,
    0.000003452163,
    0.000003218302,
    0.000003000300,
    0.000002797139,
    0.000002607800,
    0.000002431220,
    0.000002266531,
    0.000002113013,
    0.000001969943,
    0.000001836600,
    0.000001712230,
    0.000001596228,
    0.000001488090,
    0.000001387314,
    0.000001293400,
    0.000001205820,
    0.000001124143,
    0.000001048009,
    0.0000009770578,
    0.0000009109300,
    0.0000008492513,
    0.0000007917212,
    0.0000007380904,
    0.0000006881098,
    0.0000006415300,
    0.0000005980895,
    0.0000005575746,
    0.0000005198080,
    0.0000004846123,
    0.0000004518100,
];

pub const CIE_Z: [Float; N_CIE_SAMPLES as usize] = [
    // CIE Z function values
    0.0006061000,
    0.0006808792,
    0.0007651456,
    0.0008600124,
    0.0009665928,
    0.001086000,
    0.001220586,
    0.001372729,
    0.001543579,
    0.001734286,
    0.001946000,
    0.002177777,
    0.002435809,
    0.002731953,
    0.003078064,
    0.003486000,
    0.003975227,
    0.004540880,
    0.005158320,
    0.005802907,
    0.006450001,
    0.007083216,
    0.007745488,
    0.008501152,
    0.009414544,
    0.01054999,
    0.01196580,
    0.01365587,
    0.01558805,
    0.01773015,
    0.02005001,
    0.02251136,
    0.02520288,
    0.02827972,
    0.03189704,
    0.03621000,
    0.04143771,
    0.04750372,
    0.05411988,
    0.06099803,
    0.06785001,
    0.07448632,
    0.08136156,
    0.08915364,
    0.09854048,
    0.1102000,
    0.1246133,
    0.1417017,
    0.1613035,
    0.1832568,
    0.2074000,
    0.2336921,
    0.2626114,
    0.2947746,
    0.3307985,
    0.3713000,
    0.4162091,
    0.4654642,
    0.5196948,
    0.5795303,
    0.6456000,
    0.7184838,
    0.7967133,
    0.8778459,
    0.9594390,
    1.0390501,
    1.1153673,
    1.1884971,
    1.2581233,
    1.3239296,
    1.3856000,
    1.4426352,
    1.4948035,
    1.5421903,
    1.5848807,
    1.6229600,
    1.6564048,
    1.6852959,
    1.7098745,
    1.7303821,
    1.7470600,
    1.7600446,
    1.7696233,
    1.7762637,
    1.7804334,
    1.7826000,
    1.7829682,
    1.7816998,
    1.7791982,
    1.7758671,
    1.7721100,
    1.7682589,
    1.7640390,
    1.7589438,
    1.7524663,
    1.7441000,
    1.7335595,
    1.7208581,
    1.7059369,
    1.6887372,
    1.6692000,
    1.6475287,
    1.6234127,
    1.5960223,
    1.5645280,
    1.5281000,
    1.4861114,
    1.4395215,
    1.3898799,
    1.3387362,
    1.2876400,
    1.2374223,
    1.1878243,
    1.1387611,
    1.0901480,
    1.0419000,
    0.9941976,
    0.9473473,
    0.9014531,
    0.8566193,
    0.8129501,
    0.7705173,
    0.7294448,
    0.6899136,
    0.6521049,
    0.6162000,
    0.5823286,
    0.5504162,
    0.5203376,
    0.4919673,
    0.4651800,
    0.4399246,
    0.4161836,
    0.3938822,
    0.3729459,
    0.3533000,
    0.3348578,
    0.3175521,
    0.3013375,
    0.2861686,
    0.2720000,
    0.2588171,
    0.2464838,
    0.2347718,
    0.2234533,
    0.2123000,
    0.2011692,
    0.1901196,
    0.1792254,
    0.1685608,
    0.1582000,
    0.1481383,
    0.1383758,
    0.1289942,
    0.1200751,
    0.1117000,
    0.1039048,
    0.09666748,
    0.08998272,
    0.08384531,
    0.07824999,
    0.07320899,
    0.06867816,
    0.06456784,
    0.06078835,
    0.05725001,
    0.05390435,
    0.05074664,
    0.04775276,
    0.04489859,
    0.04216000,
    0.03950728,
    0.03693564,
    0.03445836,
    0.03208872,
    0.02984000,
    0.02771181,
    0.02569444,
    0.02378716,
    0.02198925,
    0.02030000,
    0.01871805,
    0.01724036,
    0.01586364,
    0.01458461,
    0.01340000,
    0.01230723,
    0.01130188,
    0.01037792,
    0.009529306,
    0.008749999,
    0.008035200,
    0.007381600,
    0.006785400,
    0.006242800,
    0.005749999,
    0.005303600,
    0.004899800,
    0.004534200,
    0.004202400,
    0.003900000,
    0.003623200,
    0.003370600,
    0.003141400,
    0.002934800,
    0.002749999,
    0.002585200,
    0.002438600,
    0.002309400,
    0.002196800,
    0.002100000,
    0.002017733,
    0.001948200,
    0.001889800,
    0.001840933,
    0.001800000,
    0.001766267,
    0.001737800,
    0.001711200,
    0.001683067,
    0.001650001,
    0.001610133,
    0.001564400,
    0.001513600,
    0.001458533,
    0.001400000,
    0.001336667,
    0.001270000,
    0.001205000,
    0.001146667,
    0.001100000,
    0.001068800,
    0.001049400,
    0.001035600,
    0.001021200,
    0.001000000,
    0.0009686400,
    0.0009299200,
    0.0008868800,
    0.0008425600,
    0.0008000000,
    0.0007609600,
    0.0007236800,
    0.0006859200,
    0.0006454400,
    0.0006000000,
    0.0005478667,
    0.0004916000,
    0.0004354000,
    0.0003834667,
    0.0003400000,
    0.0003072533,
    0.0002831600,
    0.0002654400,
    0.0002518133,
    0.0002400000,
    0.0002295467,
    0.0002206400,
    0.0002119600,
    0.0002021867,
    0.0001900000,
    0.0001742133,
    0.0001556400,
    0.0001359600,
    0.0001168533,
    0.0001000000,
    0.00008613333,
    0.00007460000,
    0.00006500000,
    0.00005693333,
    0.00004999999,
    0.00004416000,
    0.00003948000,
    0.00003572000,
    0.00003264000,
    0.00003000000,
    0.00002765333,
    0.00002556000,
    0.00002364000,
    0.00002181333,
    0.00002000000,
    0.00001813333,
    0.00001620000,
    0.00001420000,
    0.00001213333,
    0.00001000000,
    0.000007733333,
    0.000005400000,
    0.000003200000,
    0.000001333333,
    0.000000000000,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
];

pub const CIE_LAMBDA: [Float; N_CIE_SAMPLES as usize] = [
    360.0, 361.0, 362.0, 363.0, 364.0, 365.0, 366.0, 367.0, 368.0, 369.0, 370.0, 371.0, 372.0,
    373.0, 374.0, 375.0, 376.0, 377.0, 378.0, 379.0, 380.0, 381.0, 382.0, 383.0, 384.0, 385.0,
    386.0, 387.0, 388.0, 389.0, 390.0, 391.0, 392.0, 393.0, 394.0, 395.0, 396.0, 397.0, 398.0,
    399.0, 400.0, 401.0, 402.0, 403.0, 404.0, 405.0, 406.0, 407.0, 408.0, 409.0, 410.0, 411.0,
    412.0, 413.0, 414.0, 415.0, 416.0, 417.0, 418.0, 419.0, 420.0, 421.0, 422.0, 423.0, 424.0,
    425.0, 426.0, 427.0, 428.0, 429.0, 430.0, 431.0, 432.0, 433.0, 434.0, 435.0, 436.0, 437.0,
    438.0, 439.0, 440.0, 441.0, 442.0, 443.0, 444.0, 445.0, 446.0, 447.0, 448.0, 449.0, 450.0,
    451.0, 452.0, 453.0, 454.0, 455.0, 456.0, 457.0, 458.0, 459.0, 460.0, 461.0, 462.0, 463.0,
    464.0, 465.0, 466.0, 467.0, 468.0, 469.0, 470.0, 471.0, 472.0, 473.0, 474.0, 475.0, 476.0,
    477.0, 478.0, 479.0, 480.0, 481.0, 482.0, 483.0, 484.0, 485.0, 486.0, 487.0, 488.0, 489.0,
    490.0, 491.0, 492.0, 493.0, 494.0, 495.0, 496.0, 497.0, 498.0, 499.0, 500.0, 501.0, 502.0,
    503.0, 504.0, 505.0, 506.0, 507.0, 508.0, 509.0, 510.0, 511.0, 512.0, 513.0, 514.0, 515.0,
    516.0, 517.0, 518.0, 519.0, 520.0, 521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0,
    529.0, 530.0, 531.0, 532.0, 533.0, 534.0, 535.0, 536.0, 537.0, 538.0, 539.0, 540.0, 541.0,
    542.0, 543.0, 544.0, 545.0, 546.0, 547.0, 548.0, 549.0, 550.0, 551.0, 552.0, 553.0, 554.0,
    555.0, 556.0, 557.0, 558.0, 559.0, 560.0, 561.0, 562.0, 563.0, 564.0, 565.0, 566.0, 567.0,
    568.0, 569.0, 570.0, 571.0, 572.0, 573.0, 574.0, 575.0, 576.0, 577.0, 578.0, 579.0, 580.0,
    581.0, 582.0, 583.0, 584.0, 585.0, 586.0, 587.0, 588.0, 589.0, 590.0, 591.0, 592.0, 593.0,
    594.0, 595.0, 596.0, 597.0, 598.0, 599.0, 600.0, 601.0, 602.0, 603.0, 604.0, 605.0, 606.0,
    607.0, 608.0, 609.0, 610.0, 611.0, 612.0, 613.0, 614.0, 615.0, 616.0, 617.0, 618.0, 619.0,
    620.0, 621.0, 622.0, 623.0, 624.0, 625.0, 626.0, 627.0, 628.0, 629.0, 630.0, 631.0, 632.0,
    633.0, 634.0, 635.0, 636.0, 637.0, 638.0, 639.0, 640.0, 641.0, 642.0, 643.0, 644.0, 645.0,
    646.0, 647.0, 648.0, 649.0, 650.0, 651.0, 652.0, 653.0, 654.0, 655.0, 656.0, 657.0, 658.0,
    659.0, 660.0, 661.0, 662.0, 663.0, 664.0, 665.0, 666.0, 667.0, 668.0, 669.0, 670.0, 671.0,
    672.0, 673.0, 674.0, 675.0, 676.0, 677.0, 678.0, 679.0, 680.0, 681.0, 682.0, 683.0, 684.0,
    685.0, 686.0, 687.0, 688.0, 689.0, 690.0, 691.0, 692.0, 693.0, 694.0, 695.0, 696.0, 697.0,
    698.0, 699.0, 700.0, 701.0, 702.0, 703.0, 704.0, 705.0, 706.0, 707.0, 708.0, 709.0, 710.0,
    711.0, 712.0, 713.0, 714.0, 715.0, 716.0, 717.0, 718.0, 719.0, 720.0, 721.0, 722.0, 723.0,
    724.0, 725.0, 726.0, 727.0, 728.0, 729.0, 730.0, 731.0, 732.0, 733.0, 734.0, 735.0, 736.0,
    737.0, 738.0, 739.0, 740.0, 741.0, 742.0, 743.0, 744.0, 745.0, 746.0, 747.0, 748.0, 749.0,
    750.0, 751.0, 752.0, 753.0, 754.0, 755.0, 756.0, 757.0, 758.0, 759.0, 760.0, 761.0, 762.0,
    763.0, 764.0, 765.0, 766.0, 767.0, 768.0, 769.0, 770.0, 771.0, 772.0, 773.0, 774.0, 775.0,
    776.0, 777.0, 778.0, 779.0, 780.0, 781.0, 782.0, 783.0, 784.0, 785.0, 786.0, 787.0, 788.0,
    789.0, 790.0, 791.0, 792.0, 793.0, 794.0, 795.0, 796.0, 797.0, 798.0, 799.0, 800.0, 801.0,
    802.0, 803.0, 804.0, 805.0, 806.0, 807.0, 808.0, 809.0, 810.0, 811.0, 812.0, 813.0, 814.0,
    815.0, 816.0, 817.0, 818.0, 819.0, 820.0, 821.0, 822.0, 823.0, 824.0, 825.0, 826.0, 827.0,
    828.0, 829.0, 830.0,
];
pub const CIE_Y_INTEGRAL: Float = 106.856895;

pub fn blackbody(lambda: &[Float], n: usize, t: Float, le: &mut Vec<Float>) {
    if t <= 0.0 as Float {
        for _i in 0..n {
            le.push(0.0 as Float);
        }
        return;
    }
    let c: Float = 299792458.0 as Float;
    let h: Float = 6.62606957e-34 as Float;
    let kb: Float = 1.3806488e-23 as Float;
    for i in 0..n {
        let l: Float = lambda[i] * 1.0e-9 as Float;
        let lambda5: Float = (l * l) * (l * l) * l;
        let lei: Float = (2.0 as Float * h * c * c)
            / (lambda5 * (((h * c) / (l * kb * t)).exp() - 1.0 as Float));
        assert!(!lei.is_nan());
        le.push(lei);
    }
}

pub fn blackbody_normalized(lambda: &[Float], n: usize, t: Float, le: &mut Vec<Float>) {
    blackbody(lambda, n, t, le);
    // normalize _Le_ values based on maximum blackbody radiance
    let lambda_max: [Float; 1] = [2.8977721e-3 as Float / t * 1.0e9 as Float];
    let mut max_l: Vec<Float> = Vec::new();
    blackbody(&lambda_max, 1, t, &mut max_l);
    for i in 0..n {
        le[i] /= max_l[0];
    }
}

#[derive(Debug, Clone)]
pub enum SpectrumType {
    Reflectance,
    Illuminant,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RGBSpectrum {
    pub c: [Float; 3],
}

impl RGBSpectrum {
    pub fn new(v: Float) -> Self {
        // let n_spectrum_samples = 3; // RGB
        RGBSpectrum { c: [v, v, v] }
        // TODO: DCHECK(!HasNaNs());
    }
    pub fn rgb(r: Float, g: Float, b: Float) -> RGBSpectrum {
        RGBSpectrum { c: [r, g, b] }
    }
    pub fn from_srgb(rgb: &[u8; 3]) -> RGBSpectrum {
        fn convert(v: u8) -> Float {
            let value = v as Float / 255.0;
            // see InverseGammaCorrect(Float value) in pbrt.h
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) * 1.0 / 1.055).powf(2.4)
            }
        }
        RGBSpectrum::rgb(convert(rgb[0]), convert(rgb[1]), convert(rgb[2]))
    }
    pub fn from_rgb(rgb: &[Float; 3]) -> RGBSpectrum {
        let mut s: RGBSpectrum = RGBSpectrum::new(0.0 as Float);
        s.c[0] = rgb[0];
        s.c[1] = rgb[1];
        s.c[2] = rgb[2];
        // TODO: DCHECK(!s.HasNaNs());
        s
    }
    pub fn to_rgb(&self, rgb: &mut [Float; 3]) {
        rgb[0] = self.c[0];
        rgb[1] = self.c[1];
        rgb[2] = self.c[2];
    }
    pub fn to_xyz(&self, xyz: &mut [Float; 3]) {
        rgb_to_xyz(&self.c, xyz);
    }
    pub fn from_xyz(xyz: &[Float; 3], _spectrum_type: SpectrumType) -> RGBSpectrum {
        let mut r: RGBSpectrum = RGBSpectrum::new(0.0 as Float);
        xyz_to_rgb(xyz, &mut r.c);
        r
    }
    pub fn y(&self) -> Float {
        let y_weight: [Float; 3] = [0.212671, 0.715160, 0.072169];
        y_weight[0] * self.c[0] + y_weight[1] * self.c[1] + y_weight[2] * self.c[2]
    }
    pub fn from_sampled(lambda: &[Float], v: &[Float], n: i32) -> RGBSpectrum {
        // sort samples if unordered, use sorted for returned spectrum
        if !spectrum_samples_sorted(lambda, v, n) {
            panic!("TODO: if !spectrum_samples_sorted(...) {...}");
            // std::vector<Float> slambda(&lambda[0], &lambda[n]);
            // std::vector<Float> sv(&v[0], &v[n]);
            // SortSpectrumSamples(&slambda[0], &sv[0], n);
            // return FromSampled(&slambda[0], &sv[0], n);
        }
        let mut xyz: [Float; 3] = [0.0 as Float; 3];
        for i in 0..N_CIE_SAMPLES {
            let val: Float = interpolate_spectrum_samples(lambda, v, n, CIE_LAMBDA[i as usize]);
            xyz[0] += val * CIE_X[i as usize];
            xyz[1] += val * CIE_Y[i as usize];
            xyz[2] += val * CIE_Z[i as usize];
        }
        let scale: Float = (CIE_LAMBDA[(N_CIE_SAMPLES - 1) as usize] - CIE_LAMBDA[0]) as Float
            / (CIE_Y_INTEGRAL * N_CIE_SAMPLES as Float);
        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;
        RGBSpectrum::from_xyz(&xyz, SpectrumType::Reflectance)
    }
    // from CoefficientSpectrum
    pub fn is_black(&self) -> bool {
        for i in 0..3 {
            if self.c[i] != 0.0 as Float {
                return false;
            }
        }
        true
    }
    pub fn sqrt(&self) -> RGBSpectrum {
        RGBSpectrum::rgb(self.c[0].sqrt(), self.c[1].sqrt(), self.c[2].sqrt())
    }
    pub fn exp(&self) -> RGBSpectrum {
        RGBSpectrum::rgb(self.c[0].exp(), self.c[1].exp(), self.c[2].exp())
    }
    /// Clamp spectrum to lie between the values low and high. Use
    /// (0.0 as Float, std::f32::INFINITY as Float) if there are no
    /// specific values.
    pub fn clamp(&self, low: Float, high: Float) -> RGBSpectrum {
        let mut ret: RGBSpectrum = RGBSpectrum::default();
        let n_spectrum_samples: usize = 3; // RGB
        for i in 0..n_spectrum_samples {
            ret.c[i] = clamp_t(self.c[i], low, high);
        }
        assert!(!ret.has_nans());
        ret
    }
    pub fn max_component_value(&self) -> Float {
        let mut m: Float = self.c[0];
        let n_spectrum_samples: usize = 3; // RGB
        for i in 1..n_spectrum_samples {
            m = m.max(self.c[i]);
        }
        m
    }
    pub fn has_nans(&self) -> bool {
        for i in 0..3 {
            if self.c[i].is_nan() {
                return true;
            }
        }
        false
    }
}

impl PartialEq for RGBSpectrum {
    fn eq(&self, rhs: &RGBSpectrum) -> bool {
        for i in 0..3 {
            if self.c[i] != rhs.c[i] {
                return false;
            }
        }
        true
    }
}

impl Add for RGBSpectrum {
    type Output = RGBSpectrum;
    fn add(self, rhs: RGBSpectrum) -> RGBSpectrum {
        RGBSpectrum {
            c: [
                self.c[0] + rhs.c[0],
                self.c[1] + rhs.c[1],
                self.c[2] + rhs.c[2],
            ],
        }
    }
}

impl AddAssign for RGBSpectrum {
    fn add_assign(&mut self, rhs: RGBSpectrum) {
        // TODO: DCHECK(!s2.HasNaNs());
        self.c[0] += rhs.c[0];
        self.c[1] += rhs.c[1];
        self.c[2] += rhs.c[2];
    }
}

impl Mul for RGBSpectrum {
    type Output = RGBSpectrum;
    fn mul(self, rhs: RGBSpectrum) -> RGBSpectrum {
        RGBSpectrum {
            c: [
                self.c[0] * rhs.c[0],
                self.c[1] * rhs.c[1],
                self.c[2] * rhs.c[2],
            ],
        }
    }
}

impl Mul<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn mul(self, rhs: Float) -> RGBSpectrum {
        RGBSpectrum {
            c: [self.c[0] * rhs, self.c[1] * rhs, self.c[2] * rhs],
        }
    }
}

impl MulAssign for RGBSpectrum {
    fn mul_assign(&mut self, rhs: RGBSpectrum) {
        // TODO: DCHECK(!HasNaNs());
        self.c[0] *= rhs.c[0];
        self.c[1] *= rhs.c[1];
        self.c[2] *= rhs.c[2];
    }
}

impl Sub for RGBSpectrum {
    type Output = RGBSpectrum;
    fn sub(self, rhs: RGBSpectrum) -> RGBSpectrum {
        RGBSpectrum {
            c: [
                self.c[0] - rhs.c[0],
                self.c[1] - rhs.c[1],
                self.c[2] - rhs.c[2],
            ],
        }
    }
}

impl Div for RGBSpectrum {
    type Output = RGBSpectrum;
    fn div(self, rhs: RGBSpectrum) -> RGBSpectrum {
        RGBSpectrum {
            c: [
                self.c[0] / rhs.c[0],
                self.c[1] / rhs.c[1],
                self.c[2] / rhs.c[2],
            ],
        }
    }
}

impl Div<Float> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn div(self, rhs: Float) -> RGBSpectrum {
        assert_ne!(rhs, 0.0 as Float);
        assert!(!rhs.is_nan(), "rhs is NaN");
        let ret: RGBSpectrum = RGBSpectrum {
            c: [self.c[0] / rhs, self.c[1] / rhs, self.c[2] / rhs],
        };
        assert!(!ret.has_nans());
        ret
    }
}

impl Neg for RGBSpectrum {
    type Output = RGBSpectrum;
    fn neg(self) -> RGBSpectrum {
        RGBSpectrum {
            c: [-self.c[0], -self.c[1], -self.c[2]],
        }
    }
}

impl Zero for RGBSpectrum {
    fn zero() -> RGBSpectrum {
        RGBSpectrum::new(0.0 as Float)
    }

    fn is_zero(&self) -> bool {
        self.is_black()
    }
}

impl Index<usize> for RGBSpectrum {
    type Output = Float;
    fn index(&self, index: usize) -> &Float {
        match index {
            0 => &self.c[0],
            1 => &self.c[1],
            2 => &self.c[2],
            _ => panic!("Check failed: i >= 0 && i <= 2"),
        }
    }
}

impl IndexMut<usize> for RGBSpectrum {
    fn index_mut(&mut self, index: usize) -> &mut Float {
        match index {
            0 => &mut self.c[0],
            1 => &mut self.c[1],
            2 => &mut self.c[2],
            _ => panic!("Check failed: i >= 0 && i <= 2"),
        }
    }
}

impl From<Float> for RGBSpectrum {
    fn from(f: Float) -> Self {
        RGBSpectrum::new(f)
    }
}

/// Calculate RGB coefficients from a XYZ representation.
pub fn xyz_to_rgb(xyz: &[Float; 3], rgb: &mut [Float; 3]) {
    rgb[0] = 3.240479 * xyz[0] - 1.537150 * xyz[1] - 0.498535 * xyz[2];
    rgb[1] = -0.969256 * xyz[0] + 1.875991 * xyz[1] + 0.041556 * xyz[2];
    rgb[2] = 0.055648 * xyz[0] - 0.204043 * xyz[1] + 1.057311 * xyz[2];
}

/// Calculate XYZ representation from RGB coefficients.
pub fn rgb_to_xyz(rgb: &[Float; 3], xyz: &mut [Float; 3]) {
    xyz[0] = 0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2];
    xyz[1] = 0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2];
    xyz[2] = 0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2];
}

/// Interpolate linearly between two provided values.
pub fn lerp_rgb(t: Float, s1: &Spectrum, s2: &Spectrum) -> Spectrum {
    *s1 * (1.0 as Float - t) + *s2 * t
}

// see spectrum.cpp

/// Are the values sorted by wavelength?
pub fn spectrum_samples_sorted(lambda: &[Float], _vals: &[Float], n: i32) -> bool {
    for i in 0..(n - 1) {
        if lambda[i as usize] > lambda[(i + 1) as usize] {
            return false;
        }
    }
    true
}

/// Find responsible interval and linearly interpolate between the two
/// sample values.
pub fn interpolate_spectrum_samples(lambda: &[Float], vals: &[Float], n: i32, l: Float) -> Float {
    for i in 0..(n - 1) {
        assert!(lambda[(i + 1) as usize] > lambda[i as usize]);
    }
    if l <= lambda[0] {
        return vals[0];
    }
    if l >= lambda[(n - 1) as usize] {
        return vals[(n - 1) as usize];
    }
    let offset: usize = find_interval(n as usize, |index| lambda[index as usize] <= l);
    assert!(l >= lambda[offset] && l <= lambda[offset + 1]);
    let t: Float = (l - lambda[offset]) / (lambda[offset + 1] - lambda[offset]);
    lerp(t, vals[offset], vals[offset + 1])
}
