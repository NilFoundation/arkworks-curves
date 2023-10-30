use crate::{Fq, Fr};
#[cfg(not(feature = "zkllvm"))]
use crate::g1;
use ark_ec::{
    hashing::curve_maps::swu::SWUConfig,
    models::{
        short_weierstrass::{Affine, SWCurveConfig},
        CurveConfig,
    },
};
#[cfg(not(feature = "zkllvm"))]
use ark_ec::hashing::curve_maps::wb::IsogenyMap;
use ark_ff::MontFp;

type G1Affine = Affine<SwuIsoConfig>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct SwuIsoConfig;

impl CurveConfig for SwuIsoConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x8c00aaab0000aaab, 0x396c8c005555e156];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    const COFACTOR_INV: Fr =
        MontFp!("52435875175126190458656871551744051925719901746859129887267498875565241663483");
}

// https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/
// Hashing to Elliptic Curves
// 8.8.1.  BLS12-381 G1
// BLS12381G1_XMD:SHA-256_SSWU_RO_ is defined as follows:
// * E': y'^2 = x'^3 + A' * x' + B', where
//      -  A' = 0x144698a3b8e9433d693a02c96d4982b0ea985383ee66a8d8e8981aefd881ac98936f8da0e0f97f5cf428082d584c1d
//      -  B' = 0x12e2908d11688030018b12e8753eee3b2016c1f0f24f4070a0b9c14fcef35ef55a23215a316ceaa5d1cc48e98e172be0
//      -  A' = 12190336318893619529228877361869031420615612348429846051986726275283378313155663745811710833465465981901188123677
//      -  B' = 2906670324641927570491258158026293881577086121416628140204402091718288198173574630967936031029026176254968826637280
//  * Z: 11
impl SWCurveConfig for SwuIsoConfig {
    const COEFF_A: Fq = MontFp!("12190336318893619529228877361869031420615612348429846051986726275283378313155663745811710833465465981901188123677");

    #[rustfmt::skip]
    const COEFF_B: Fq = MontFp!("2906670324641927570491258158026293881577086121416628140204402091718288198173574630967936031029026176254968826637280");

    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}

/// Lexicographically smallest, valid x-coordinate of a point P on the curve
/// (with its corresponding y) multiplied by the cofactor. P_x = 2
/// P_y = 658522096176515125667361255350269797307718222519385801637008089782287711363858559738763090642304321670226247205569
/// P = E(P_x, P_y)
/// G = P * COFACTOR
const G1_GENERATOR_X: Fq = MontFp!("1677416608493238977774703213729589714082762656433187746258164626835771660734158898989765932111853529350617333597651");
const G1_GENERATOR_Y: Fq = MontFp!("1405098061573104639413728190240719229571583960971553962991897960445246185035342568402755187331334546673157015627211");

impl SWUConfig for SwuIsoConfig {
    // ZETA = 0xb as per the IETF draft.
    const ZETA: Fq = MontFp!("11");
}

#[cfg(not(feature = "zkllvm"))]
pub const ISOGENY_MAP_TO_G1 : IsogenyMap<'_, SwuIsoConfig, g1::Config, > = IsogenyMap {
    x_map_numerator : &[
        MontFp!("2712959285290305970661081772124144179193819192423276218370281158706191519995889425075952244140278856085036081760695"),
        MontFp!("3564859427549639835253027846704205725951033235539816243131874237388832081954622352624080767121604606753339903542203"),
        MontFp!("2051387046688339481714726479723076305756384619135044672831882917686431912682625619320120082313093891743187631791280"),
        MontFp!("3612713941521031012780325893181011392520079402153354595775735142359240110423346445050803899623018402874731133626465"),
        MontFp!("2247053637822768981792833880270996398470828564809439728372634811976089874056583714987807553397615562273407692740057"),
        MontFp!("3415427104483187489859740871640064348492611444552862448295571438270821994900526625562705192993481400731539293415811"),
        MontFp!("2067521456483432583860405634125513059912765526223015704616050604591207046392807563217109432457129564962571408764292"),
        MontFp!("3650721292069012982822225637849018828271936405382082649291891245623305084633066170122780668657208923883092359301262"),
        MontFp!("1239271775787030039269460763652455868148971086016832054354147730155061349388626624328773377658494412538595239256855"),
        MontFp!("3479374185711034293956731583912244564891370843071137483962415222733470401948838363051960066766720884717833231600798"),
        MontFp!("2492756312273161536685660027440158956721981129429869601638362407515627529461742974364729223659746272460004902959995"),
        MontFp!("1058488477413994682556770863004536636444795456512795473806825292198091015005841418695586811009326456605062948114985"),
    ],

    x_map_denominator : &[
        MontFp!("1353092447850172218905095041059784486169131709710991428415161466575141675351394082965234118340787683181925558786844"),
        MontFp!("2822220997908397120956501031591772354860004534930174057793539372552395729721474912921980407622851861692773516917759"),
        MontFp!("1717937747208385987946072944131378949849282930538642983149296304709633281382731764122371874602115081850953846504985"),
        MontFp!("501624051089734157816582944025690868317536915684467868346388760435016044027032505306995281054569109955275640941784"),
        MontFp!("3025903087998593826923738290305187197829899948335370692927241015584233559365859980023579293766193297662657497834014"),
        MontFp!("2224140216975189437834161136818943039444741035168992629437640302964164227138031844090123490881551522278632040105125"),
        MontFp!("1146414465848284837484508420047674663876992808692209238763293935905506532411661921697047880549716175045414621825594"),
        MontFp!("3179090966864399634396993677377903383656908036827452986467581478509513058347781039562481806409014718357094150199902"),
        MontFp!("1549317016540628014674302140786462938410429359529923207442151939696344988707002602944342203885692366490121021806145"),
        MontFp!("1442797143427491432630626390066422021593505165588630398337491100088557278058060064930663878153124164818522816175370"),
        MontFp!("1"),
    ],

    y_map_numerator : &[
        MontFp!("1393399195776646641963150658816615410692049723305861307490980409834842911816308830479576739332720113414154429643571"),
        MontFp!("2968610969752762946134106091152102846225411740689724909058016729455736597929366401532929068084731548131227395540630"),
        MontFp!("122933100683284845219599644396874530871261396084070222155796123161881094323788483360414289333111221370374027338230"),
        MontFp!("303251954782077855462083823228569901064301365507057490567314302006681283228886645653148231378803311079384246777035"),
        MontFp!("1353972356724735644398279028378555627591260676383150667237975415318226973994509601413730187583692624416197017403099"),
        MontFp!("3443977503653895028417260979421240655844034880950251104724609885224259484262346958661845148165419691583810082940400"),
        MontFp!("718493410301850496156792713845282235942975872282052335612908458061560958159410402177452633054233549648465863759602"),
        MontFp!("1466864076415884313141727877156167508644960317046160398342634861648153052436926062434809922037623519108138661903145"),
        MontFp!("1536886493137106337339531461344158973554574987550750910027365237255347020572858445054025958480906372033954157667719"),
        MontFp!("2171468288973248519912068884667133903101171670397991979582205855298465414047741472281361964966463442016062407908400"),
        MontFp!("3915937073730221072189646057898966011292434045388986394373682715266664498392389619761133407846638689998746172899634"),
        MontFp!("3802409194827407598156407709510350851173404795262202653149767739163117554648574333789388883640862266596657730112910"),
        MontFp!("1707589313757812493102695021134258021969283151093981498394095062397393499601961942449581422761005023512037430861560"),
        MontFp!("349697005987545415860583335313370109325490073856352967581197273584891698473628451945217286148025358795756956811571"),
        MontFp!("885704436476567581377743161796735879083481447641210566405057346859953524538988296201011389016649354976986251207243"),
        MontFp!("3370924952219000111210625390420697640496067348723987858345031683392215988129398381698161406651860675722373763741188"),
    ],

    y_map_denominator : &[
        MontFp!("3396434800020507717552209507749485772788165484415495716688989613875369612529138640646200921379825018840894888371137"),
        MontFp!("3907278185868397906991868466757978732688957419873771881240086730384895060595583602347317992689443299391009456758845"),
        MontFp!("854914566454823955479427412036002165304466268547334760894270240966182605542146252771872707010378658178126128834546"),
        MontFp!("3496628876382137961119423566187258795236027183112131017519536056628828830323846696121917502443333849318934945158166"),
        MontFp!("1828256966233331991927609917644344011503610008134915752990581590799656305331275863706710232159635159092657073225757"),
        MontFp!("1362317127649143894542621413133849052553333099883364300946623208643344298804722863920546222860227051989127113848748"),
        MontFp!("3443845896188810583748698342858554856823966611538932245284665132724280883115455093457486044009395063504744802318172"),
        MontFp!("3484671274283470572728732863557945897902920439975203610275006103818288159899345245633896492713412187296754791689945"),
        MontFp!("3755735109429418587065437067067640634211015783636675372165599470771975919172394156249639331555277748466603540045130"),
        MontFp!("3459661102222301807083870307127272890283709299202626530836335779816726101522661683404130556379097384249447658110805"),
        MontFp!("742483168411032072323733249644347333168432665415341249073150659015707795549260947228694495111018381111866512337576"),
        MontFp!("1662231279858095762833829698537304807741442669992646287950513237989158777254081548205552083108208170765474149568658"),
        MontFp!("1668238650112823419388205992952852912407572045257706138925379268508860023191233729074751042562151098884528280913356"),
        MontFp!("369162719928976119195087327055926326601627748362769544198813069133429557026740823593067700396825489145575282378487"),
        MontFp!("2164195715141237148945939585099633032390257748382945597506236650132835917087090097395995817229686247227784224263055"),
        MontFp!("1"),
    ],
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen() {
        let gen: G1Affine = SwuIsoConfig::GENERATOR;
        assert!(gen.is_on_curve());
        assert!(gen.is_in_correct_subgroup_assuming_on_curve());
    }
}
