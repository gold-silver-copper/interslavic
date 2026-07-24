use phf::phf_map;
use super::VerbDictionaryEntry;

pub(crate) static VERB_METADATA: phf::Map<&'static str, &'static [VerbDictionaryEntry]> = phf_map! {
    "#doględati" => &[
        VerbDictionaryEntry { lemma: "#doględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#goniti" => &[
        VerbDictionaryEntry { lemma: "#goniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#izdělati" => &[
        VerbDictionaryEntry { lemma: "#izdělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#izkljuvati" => &[
        VerbDictionaryEntry { lemma: "#izkljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#izobličati" => &[
        VerbDictionaryEntry { lemma: "#izobličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#izstųpiti" => &[
        VerbDictionaryEntry { lemma: "#izstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "#objavjati sę" => &[
        VerbDictionaryEntry { lemma: "#objavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "#oblěkati" => &[
        VerbDictionaryEntry { lemma: "#oblěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#obstanoviti" => &[
        VerbDictionaryEntry { lemma: "#obstanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#ocěnjati" => &[
        VerbDictionaryEntry { lemma: "#ocěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#odprašati sę" => &[
        VerbDictionaryEntry { lemma: "#odprašati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "#odtiskati" => &[
        VerbDictionaryEntry { lemma: "#odtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#orositi" => &[
        VerbDictionaryEntry { lemma: "#orositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#ovoščiti" => &[
        VerbDictionaryEntry { lemma: "#ovoščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#oznaniti" => &[
        VerbDictionaryEntry { lemma: "#oznaniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#pnųti" => &[
        VerbDictionaryEntry { lemma: "#pnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#posmotriti" => &[
        VerbDictionaryEntry { lemma: "#posmotriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "#pretendovati" => &[
        VerbDictionaryEntry { lemma: "#pretendovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#prědpostavjati" => &[
        VerbDictionaryEntry { lemma: "#prědpostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#prěznačati" => &[
        VerbDictionaryEntry { lemma: "#prěznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#råzstlati" => &[
        VerbDictionaryEntry { lemma: "#råzstlati", addition: "(råzstelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#smotriti" => &[
        VerbDictionaryEntry { lemma: "#smotriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "#spisati" => &[
        VerbDictionaryEntry { lemma: "#spisati", addition: "(spiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#stvŕditi" => &[
        VerbDictionaryEntry { lemma: "#stvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#sypati" => &[
        VerbDictionaryEntry { lemma: "#sypati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#sčityvati" => &[
        VerbDictionaryEntry { lemma: "#sčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#sȯžegti" => &[
        VerbDictionaryEntry { lemma: "#sȯžegti", addition: "(sȯžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#udivjati sę" => &[
        VerbDictionaryEntry { lemma: "#udivjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "#urězati" => &[
        VerbDictionaryEntry { lemma: "#urězati", addition: "(urěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#učiniti" => &[
        VerbDictionaryEntry { lemma: "#učiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#vlězti" => &[
        VerbDictionaryEntry { lemma: "#vlězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "#vsosati" => &[
        VerbDictionaryEntry { lemma: "#vsosati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#vykati" => &[
        VerbDictionaryEntry { lemma: "#vykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#vytŕpěti" => &[
        VerbDictionaryEntry { lemma: "#vytŕpěti", addition: "(vytŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#vyznavati" => &[
        VerbDictionaryEntry { lemma: "#vyznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#vȯzdvigati" => &[
        VerbDictionaryEntry { lemma: "#vȯzdvigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "#vȯzkypyvati" => &[
        VerbDictionaryEntry { lemma: "#vȯzkypyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "#zakazati" => &[
        VerbDictionaryEntry { lemma: "#zakazati", addition: "(zakaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "#zasvětiti" => &[
        VerbDictionaryEntry { lemma: "#zasvětiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "abdikovati" => &[
        VerbDictionaryEntry { lemma: "abdikovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "abonovati" => &[
        VerbDictionaryEntry { lemma: "abonovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "absolutizovati" => &[
        VerbDictionaryEntry { lemma: "absolutizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "absorbovati" => &[
        VerbDictionaryEntry { lemma: "absorbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "abstrahovati" => &[
        VerbDictionaryEntry { lemma: "abstrahovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "adaptovati" => &[
        VerbDictionaryEntry { lemma: "adaptovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "administrovati" => &[
        VerbDictionaryEntry { lemma: "administrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "adoptovati" => &[
        VerbDictionaryEntry { lemma: "adoptovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "adresovati" => &[
        VerbDictionaryEntry { lemma: "adresovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "agitovati" => &[
        VerbDictionaryEntry { lemma: "agitovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "agonizovati" => &[
        VerbDictionaryEntry { lemma: "agonizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "akcentovati" => &[
        VerbDictionaryEntry { lemma: "akcentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "akceptovati" => &[
        VerbDictionaryEntry { lemma: "akceptovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "aklimatizovati" => &[
        VerbDictionaryEntry { lemma: "aklimatizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "akompanovati" => &[
        VerbDictionaryEntry { lemma: "akompanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "aktivovati" => &[
        VerbDictionaryEntry { lemma: "aktivovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "aktualizovati" => &[
        VerbDictionaryEntry { lemma: "aktualizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "akumulovati" => &[
        VerbDictionaryEntry { lemma: "akumulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "amnestovati" => &[
        VerbDictionaryEntry { lemma: "amnestovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "amortizovati" => &[
        VerbDictionaryEntry { lemma: "amortizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "amputovati" => &[
        VerbDictionaryEntry { lemma: "amputovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "analizovati" => &[
        VerbDictionaryEntry { lemma: "analizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "aneksovati" => &[
        VerbDictionaryEntry { lemma: "aneksovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "anulovati" => &[
        VerbDictionaryEntry { lemma: "anulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "apelovati" => &[
        VerbDictionaryEntry { lemma: "apelovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "aplodovati" => &[
        VerbDictionaryEntry { lemma: "aplodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "aranževati" => &[
        VerbDictionaryEntry { lemma: "aranževati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "areštovati" => &[
        VerbDictionaryEntry { lemma: "areštovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "argumentovati" => &[
        VerbDictionaryEntry { lemma: "argumentovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "arhivovati" => &[
        VerbDictionaryEntry { lemma: "arhivovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "asimilovati" => &[
        VerbDictionaryEntry { lemma: "asimilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "atakovati" => &[
        VerbDictionaryEntry { lemma: "atakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "avansovati" => &[
        VerbDictionaryEntry { lemma: "avansovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "avtomatizovati" => &[
        VerbDictionaryEntry { lemma: "avtomatizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "avtorizovati" => &[
        VerbDictionaryEntry { lemma: "avtorizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "avtostopovati" => &[
        VerbDictionaryEntry { lemma: "avtostopovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "bagatelizovati" => &[
        VerbDictionaryEntry { lemma: "bagatelizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bajati" => &[
        VerbDictionaryEntry { lemma: "bajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "balansovati" => &[
        VerbDictionaryEntry { lemma: "balansovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "balotovati" => &[
        VerbDictionaryEntry { lemma: "balotovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "balzamovati" => &[
        VerbDictionaryEntry { lemma: "balzamovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "barikadovati" => &[
        VerbDictionaryEntry { lemma: "barikadovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "barviti" => &[
        VerbDictionaryEntry { lemma: "barviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "baviti" => &[
        VerbDictionaryEntry { lemma: "baviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bděti" => &[
        VerbDictionaryEntry { lemma: "bděti", addition: "(bdi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "besědovati" => &[
        VerbDictionaryEntry { lemma: "besědovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bezpokojiti" => &[
        VerbDictionaryEntry { lemma: "bezpokojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bezpokojiti sę" => &[
        VerbDictionaryEntry { lemma: "bezpokojiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "biti" => &[
        VerbDictionaryEntry { lemma: "biti", addition: "(bije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bičevati" => &[
        VerbDictionaryEntry { lemma: "bičevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "blazniti sę" => &[
        VerbDictionaryEntry { lemma: "blazniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "blaznovati" => &[
        VerbDictionaryEntry { lemma: "blaznovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blejati" => &[
        VerbDictionaryEntry { lemma: "blejati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bližiti sę" => &[
        VerbDictionaryEntry { lemma: "bližiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "bljunųti" => &[
        VerbDictionaryEntry { lemma: "bljunųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "bljuvati" => &[
        VerbDictionaryEntry { lemma: "bljuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blokovati" => &[
        VerbDictionaryEntry { lemma: "blokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "blågodariti" => &[
        VerbDictionaryEntry { lemma: "blågodariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "blågoslavjati" => &[
        VerbDictionaryEntry { lemma: "blågoslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "blågosloviti" => &[
        VerbDictionaryEntry { lemma: "blågosloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "blågovolěti" => &[
        VerbDictionaryEntry { lemma: "blågovolěti", addition: "(blågovoli)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blågoželati" => &[
        VerbDictionaryEntry { lemma: "blågoželati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "blědněti" => &[
        VerbDictionaryEntry { lemma: "blědněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blěskati" => &[
        VerbDictionaryEntry { lemma: "blěskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blěsknųti" => &[
        VerbDictionaryEntry { lemma: "blěsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "blěstěti" => &[
        VerbDictionaryEntry { lemma: "blěstěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blųditi" => &[
        VerbDictionaryEntry { lemma: "blųditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "blųkati" => &[
        VerbDictionaryEntry { lemma: "blųkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bodati" => &[
        VerbDictionaryEntry { lemma: "bodati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bodati sę" => &[
        VerbDictionaryEntry { lemma: "bodati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "bodnųti" => &[
        VerbDictionaryEntry { lemma: "bodnųti", addition: "(bode)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "bodnųti sę" => &[
        VerbDictionaryEntry { lemma: "bodnųti sę", addition: "(bode)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "bogatěti" => &[
        VerbDictionaryEntry { lemma: "bogatěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bogohuliti" => &[
        VerbDictionaryEntry { lemma: "bogohuliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bojati sę" => &[
        VerbDictionaryEntry { lemma: "bojati sę", addition: "(boji)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "bojevati" => &[
        VerbDictionaryEntry { lemma: "bojevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bojkotovati" => &[
        VerbDictionaryEntry { lemma: "bojkotovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bolěti" => &[
        VerbDictionaryEntry { lemma: "bolěti", addition: "(bolěje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "bolěti", addition: "(boli)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bombardovati" => &[
        VerbDictionaryEntry { lemma: "bombardovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "boriti sę" => &[
        VerbDictionaryEntry { lemma: "boriti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "bosti" => &[
        VerbDictionaryEntry { lemma: "bosti", addition: "(bode)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bosti sę" => &[
        VerbDictionaryEntry { lemma: "bosti sę", addition: "(bode)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "brahtati sę" => &[
        VerbDictionaryEntry { lemma: "brahtati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "bratati sę" => &[
        VerbDictionaryEntry { lemma: "bratati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "brati" => &[
        VerbDictionaryEntry { lemma: "brati", addition: "(bere)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "breknųti" => &[
        VerbDictionaryEntry { lemma: "breknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "brenknųti" => &[
        VerbDictionaryEntry { lemma: "brenknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "brenčati" => &[
        VerbDictionaryEntry { lemma: "brenčati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "briti" => &[
        VerbDictionaryEntry { lemma: "briti", addition: "(brije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "briti sę" => &[
        VerbDictionaryEntry { lemma: "briti sę", addition: "(brije)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "broditi" => &[
        VerbDictionaryEntry { lemma: "broditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "brusiti" => &[
        VerbDictionaryEntry { lemma: "brusiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bryzgati" => &[
        VerbDictionaryEntry { lemma: "bryzgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bryzgnųti" => &[
        VerbDictionaryEntry { lemma: "bryzgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "bråniti" => &[
        VerbDictionaryEntry { lemma: "bråniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bråzditi" => &[
        VerbDictionaryEntry { lemma: "bråzditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "buditi" => &[
        VerbDictionaryEntry { lemma: "buditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "buditi sę" => &[
        VerbDictionaryEntry { lemma: "buditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "budovati" => &[
        VerbDictionaryEntry { lemma: "budovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "buhati" => &[
        VerbDictionaryEntry { lemma: "buhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "buhnųti" => &[
        VerbDictionaryEntry { lemma: "buhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "buntovati sę" => &[
        VerbDictionaryEntry { lemma: "buntovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "buriti" => &[
        VerbDictionaryEntry { lemma: "buriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bučati" => &[
        VerbDictionaryEntry { lemma: "bučati", addition: "(buče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "byti" => &[
        VerbDictionaryEntry { lemma: "byti", addition: "(jesm, jesi, jest, jesmo, jeste, sųt; byl; bųdų)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "byvati" => &[
        VerbDictionaryEntry { lemma: "byvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "bzdnųti" => &[
        VerbDictionaryEntry { lemma: "bzdnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "bzděti" => &[
        VerbDictionaryEntry { lemma: "bzděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "běgati" => &[
        VerbDictionaryEntry { lemma: "běgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "běgti" => &[
        VerbDictionaryEntry { lemma: "běgti", addition: "(běži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "běliti" => &[
        VerbDictionaryEntry { lemma: "běliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "bělěti" => &[
        VerbDictionaryEntry { lemma: "bělěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "běsiti sę" => &[
        VerbDictionaryEntry { lemma: "běsiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "cenzurovati" => &[
        VerbDictionaryEntry { lemma: "cenzurovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "citovati" => &[
        VerbDictionaryEntry { lemma: "citovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "cmokati" => &[
        VerbDictionaryEntry { lemma: "cmokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cviliti" => &[
        VerbDictionaryEntry { lemma: "cviliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cvěliti" => &[
        VerbDictionaryEntry { lemma: "cvěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cvěsti" => &[
        VerbDictionaryEntry { lemma: "cvěsti", addition: "(cvěte)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cvětati" => &[
        VerbDictionaryEntry { lemma: "cvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cvětnųti" => &[
        VerbDictionaryEntry { lemma: "cvětnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cvŕkati" => &[
        VerbDictionaryEntry { lemma: "cvŕkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cěditi" => &[
        VerbDictionaryEntry { lemma: "cěditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "cěliti" => &[
        VerbDictionaryEntry { lemma: "cěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "cělovati" => &[
        VerbDictionaryEntry { lemma: "cělovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "cěniti" => &[
        VerbDictionaryEntry { lemma: "cěniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dariti" => &[
        VerbDictionaryEntry { lemma: "dariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "darovati" => &[
        VerbDictionaryEntry { lemma: "darovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dati" => &[
        VerbDictionaryEntry { lemma: "dati", addition: "(da)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "datovati" => &[
        VerbDictionaryEntry { lemma: "datovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "daunlodovati" => &[
        VerbDictionaryEntry { lemma: "daunlodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "davati" => &[
        VerbDictionaryEntry { lemma: "davati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "daviti" => &[
        VerbDictionaryEntry { lemma: "daviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dbati" => &[
        VerbDictionaryEntry { lemma: "dbati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "deaktivovati" => &[
        VerbDictionaryEntry { lemma: "deaktivovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "debatovati" => &[
        VerbDictionaryEntry { lemma: "debatovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "debelěti" => &[
        VerbDictionaryEntry { lemma: "debelěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "debjutovati" => &[
        VerbDictionaryEntry { lemma: "debjutovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "deblokovati" => &[
        VerbDictionaryEntry { lemma: "deblokovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "definiovati" => &[
        VerbDictionaryEntry { lemma: "definiovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "defisovati" => &[
        VerbDictionaryEntry { lemma: "defisovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "deformovati" => &[
        VerbDictionaryEntry { lemma: "deformovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "degenerovati" => &[
        VerbDictionaryEntry { lemma: "degenerovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "degradovati" => &[
        VerbDictionaryEntry { lemma: "degradovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "degustovati" => &[
        VerbDictionaryEntry { lemma: "degustovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "deklamovati" => &[
        VerbDictionaryEntry { lemma: "deklamovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dekodovati" => &[
        VerbDictionaryEntry { lemma: "dekodovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "demonstrovati" => &[
        VerbDictionaryEntry { lemma: "demonstrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "demoralizovati" => &[
        VerbDictionaryEntry { lemma: "demoralizovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "deportovati" => &[
        VerbDictionaryEntry { lemma: "deportovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "deprimovati" => &[
        VerbDictionaryEntry { lemma: "deprimovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "destabilizovati" => &[
        VerbDictionaryEntry { lemma: "destabilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dezertovati" => &[
        VerbDictionaryEntry { lemma: "dezertovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dezinfikovati" => &[
        VerbDictionaryEntry { lemma: "dezinfikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "diktovati" => &[
        VerbDictionaryEntry { lemma: "diktovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "diskreditovati" => &[
        VerbDictionaryEntry { lemma: "diskreditovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "diskriminovati" => &[
        VerbDictionaryEntry { lemma: "diskriminovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "diskutovati" => &[
        VerbDictionaryEntry { lemma: "diskutovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "diskvalifikovati" => &[
        VerbDictionaryEntry { lemma: "diskvalifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "distancevati sę" => &[
        VerbDictionaryEntry { lemma: "distancevati sę", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "diviti" => &[
        VerbDictionaryEntry { lemma: "diviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dniti sę" => &[
        VerbDictionaryEntry { lemma: "dniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dobaviti" => &[
        VerbDictionaryEntry { lemma: "dobaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dobavjati" => &[
        VerbDictionaryEntry { lemma: "dobavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dobrěti" => &[
        VerbDictionaryEntry { lemma: "dobrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dobyti" => &[
        VerbDictionaryEntry { lemma: "dobyti", addition: "(dobųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dobyvati" => &[
        VerbDictionaryEntry { lemma: "dobyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dodati" => &[
        VerbDictionaryEntry { lemma: "dodati", addition: "(doda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dodavati" => &[
        VerbDictionaryEntry { lemma: "dodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "doganjati" => &[
        VerbDictionaryEntry { lemma: "doganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dogarjati" => &[
        VerbDictionaryEntry { lemma: "dogarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "doględěti" => &[
        VerbDictionaryEntry { lemma: "doględěti", addition: "(doględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dognati" => &[
        VerbDictionaryEntry { lemma: "dognati", addition: "(dogone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dogoditi sę" => &[
        VerbDictionaryEntry { lemma: "dogoditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dogorěti" => &[
        VerbDictionaryEntry { lemma: "dogorěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dogovarjati" => &[
        VerbDictionaryEntry { lemma: "dogovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dogovarjati sę" => &[
        VerbDictionaryEntry { lemma: "dogovarjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dogovoriti" => &[
        VerbDictionaryEntry { lemma: "dogovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dogovoriti sę" => &[
        VerbDictionaryEntry { lemma: "dogovoriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dohoditi" => &[
        VerbDictionaryEntry { lemma: "dohoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dojdti" => &[
        VerbDictionaryEntry { lemma: "dojdti", addition: "(dojde; došėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dojehati" => &[
        VerbDictionaryEntry { lemma: "dojehati", addition: "(dojede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "doježđati" => &[
        VerbDictionaryEntry { lemma: "doježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dojiti" => &[
        VerbDictionaryEntry { lemma: "dojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dokazati" => &[
        VerbDictionaryEntry { lemma: "dokazati", addition: "(dokaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dokazyvati" => &[
        VerbDictionaryEntry { lemma: "dokazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dokladati" => &[
        VerbDictionaryEntry { lemma: "dokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dokonati" => &[
        VerbDictionaryEntry { lemma: "dokonati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dokonyvati" => &[
        VerbDictionaryEntry { lemma: "dokonyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dokončiti" => &[
        VerbDictionaryEntry { lemma: "dokončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dokumentovati" => &[
        VerbDictionaryEntry { lemma: "dokumentovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dokupiti" => &[
        VerbDictionaryEntry { lemma: "dokupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dokupovati" => &[
        VerbDictionaryEntry { lemma: "dokupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "doletěti" => &[
        VerbDictionaryEntry { lemma: "doletěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "doložiti" => &[
        VerbDictionaryEntry { lemma: "doložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dominovati" => &[
        VerbDictionaryEntry { lemma: "dominovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "domněvati" => &[
        VerbDictionaryEntry { lemma: "domněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "domysliti sę" => &[
        VerbDictionaryEntry { lemma: "domysliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "domysljati sę" => &[
        VerbDictionaryEntry { lemma: "domysljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "donesti" => &[
        VerbDictionaryEntry { lemma: "donesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "donositi" => &[
        VerbDictionaryEntry { lemma: "donositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dopisati" => &[
        VerbDictionaryEntry { lemma: "dopisati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dopisyvati" => &[
        VerbDictionaryEntry { lemma: "dopisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "doplatiti" => &[
        VerbDictionaryEntry { lemma: "doplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "doplaćati" => &[
        VerbDictionaryEntry { lemma: "doplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dopustiti" => &[
        VerbDictionaryEntry { lemma: "dopustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dopušćati" => &[
        VerbDictionaryEntry { lemma: "dopušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dopȯlniti" => &[
        VerbDictionaryEntry { lemma: "dopȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dopȯlnjati" => &[
        VerbDictionaryEntry { lemma: "dopȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dopȯlzati" => &[
        VerbDictionaryEntry { lemma: "dopȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dopȯlzti" => &[
        VerbDictionaryEntry { lemma: "dopȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dorastati" => &[
        VerbDictionaryEntry { lemma: "dorastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dorysovati" => &[
        VerbDictionaryEntry { lemma: "dorysovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dorysovyvati" => &[
        VerbDictionaryEntry { lemma: "dorysovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "doråsti" => &[
        VerbDictionaryEntry { lemma: "doråsti", addition: "(doråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "doråzuměti sę" => &[
        VerbDictionaryEntry { lemma: "doråzuměti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "doråzuměvati sę" => &[
        VerbDictionaryEntry { lemma: "doråzuměvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dorųčati" => &[
        VerbDictionaryEntry { lemma: "dorųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dorųčiti" => &[
        VerbDictionaryEntry { lemma: "dorųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dosaditi" => &[
        VerbDictionaryEntry { lemma: "dosaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dosaditi sę" => &[
        VerbDictionaryEntry { lemma: "dosaditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dosađati" => &[
        VerbDictionaryEntry { lemma: "dosađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dosađati sę" => &[
        VerbDictionaryEntry { lemma: "dosađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "doskonaliti" => &[
        VerbDictionaryEntry { lemma: "doskonaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dostati" => &[
        VerbDictionaryEntry { lemma: "dostati", addition: "(dostane)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dostavati" => &[
        VerbDictionaryEntry { lemma: "dostavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dostaviti" => &[
        VerbDictionaryEntry { lemma: "dostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dostavjati" => &[
        VerbDictionaryEntry { lemma: "dostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dostigati" => &[
        VerbDictionaryEntry { lemma: "dostigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dostignųti" => &[
        VerbDictionaryEntry { lemma: "dostignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dosęgati" => &[
        VerbDictionaryEntry { lemma: "dosęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dosęgnųti" => &[
        VerbDictionaryEntry { lemma: "dosęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dotknųti" => &[
        VerbDictionaryEntry { lemma: "dotknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dotknųti sę" => &[
        VerbDictionaryEntry { lemma: "dotknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dotykati" => &[
        VerbDictionaryEntry { lemma: "dotykati", addition: "(dotyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dotykati sę" => &[
        VerbDictionaryEntry { lemma: "dotykati sę", addition: "(dotyče)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dovezti" => &[
        VerbDictionaryEntry { lemma: "dovezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dovoziti" => &[
        VerbDictionaryEntry { lemma: "dovoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dovědati sę" => &[
        VerbDictionaryEntry { lemma: "dovědati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dověděti sę" => &[
        VerbDictionaryEntry { lemma: "dověděti sę", addition: "(dově)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dověriti" => &[
        VerbDictionaryEntry { lemma: "dověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dověriti sę" => &[
        VerbDictionaryEntry { lemma: "dověriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "dověrjati" => &[
        VerbDictionaryEntry { lemma: "dověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "dověrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dověrjati sę" => &[
        VerbDictionaryEntry { lemma: "dověrjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dovŕšati" => &[
        VerbDictionaryEntry { lemma: "dovŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dovŕšiti" => &[
        VerbDictionaryEntry { lemma: "dovŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dozovati" => &[
        VerbDictionaryEntry { lemma: "dozovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dozrěti" => &[
        VerbDictionaryEntry { lemma: "dozrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dozrěvati" => &[
        VerbDictionaryEntry { lemma: "dozrěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dozvaljati" => &[
        VerbDictionaryEntry { lemma: "dozvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dozvoliti" => &[
        VerbDictionaryEntry { lemma: "dozvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dožiti" => &[
        VerbDictionaryEntry { lemma: "dožiti", addition: "(dožive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "doživati" => &[
        VerbDictionaryEntry { lemma: "doživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "drapati" => &[
        VerbDictionaryEntry { lemma: "drapati", addition: "(drape)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "drapnųti" => &[
        VerbDictionaryEntry { lemma: "drapnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dražniti" => &[
        VerbDictionaryEntry { lemma: "dražniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "drgati" => &[
        VerbDictionaryEntry { lemma: "drgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "drgnųti" => &[
        VerbDictionaryEntry { lemma: "drgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "drobiti" => &[
        VerbDictionaryEntry { lemma: "drobiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "družiti sę" => &[
        VerbDictionaryEntry { lemma: "družiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "drěmati" => &[
        VerbDictionaryEntry { lemma: "drěmati", addition: "(drěme)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "drěmnųti" => &[
        VerbDictionaryEntry { lemma: "drěmnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "drěti" => &[
        VerbDictionaryEntry { lemma: "drěti", addition: "(dre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "držati" => &[
        VerbDictionaryEntry { lemma: "držati", addition: "(drži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dubliti" => &[
        VerbDictionaryEntry { lemma: "dubliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dumati" => &[
        VerbDictionaryEntry { lemma: "dumati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "durěti" => &[
        VerbDictionaryEntry { lemma: "durěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dušiti" => &[
        VerbDictionaryEntry { lemma: "dušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dvigati" => &[
        VerbDictionaryEntry { lemma: "dvigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dvigati sę" => &[
        VerbDictionaryEntry { lemma: "dvigati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dvignųti" => &[
        VerbDictionaryEntry { lemma: "dvignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "dvojiti" => &[
        VerbDictionaryEntry { lemma: "dvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dyhati" => &[
        VerbDictionaryEntry { lemma: "dyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dyhtěti" => &[
        VerbDictionaryEntry { lemma: "dyhtěti", addition: "(dyhti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dymiti" => &[
        VerbDictionaryEntry { lemma: "dymiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dyšati" => &[
        VerbDictionaryEntry { lemma: "dyšati", addition: "(dyše)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dękovati" => &[
        VerbDictionaryEntry { lemma: "dękovati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "dějati" => &[
        VerbDictionaryEntry { lemma: "dějati", addition: "(děje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dějati sę" => &[
        VerbDictionaryEntry { lemma: "dějati sę", addition: "(děje)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dělati" => &[
        VerbDictionaryEntry { lemma: "dělati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "děliti" => &[
        VerbDictionaryEntry { lemma: "děliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "děliti sę" => &[
        VerbDictionaryEntry { lemma: "děliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "děti" => &[
        VerbDictionaryEntry { lemma: "děti", addition: "(děje/děne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "děti sę" => &[
        VerbDictionaryEntry { lemma: "děti sę", addition: "(děje/děne)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dŕkati" => &[
        VerbDictionaryEntry { lemma: "dŕkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dŕti" => &[
        VerbDictionaryEntry { lemma: "dŕti", addition: "(dre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dŕzati" => &[
        VerbDictionaryEntry { lemma: "dŕzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "dŕznųti" => &[
        VerbDictionaryEntry { lemma: "dŕznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dŕžati" => &[
        VerbDictionaryEntry { lemma: "dŕžati", addition: "(dŕži)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dųbiti" => &[
        VerbDictionaryEntry { lemma: "dųbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dųnųti" => &[
        VerbDictionaryEntry { lemma: "dųnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dųti" => &[
        VerbDictionaryEntry { lemma: "dųti", addition: "(dme)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dȯhnųti" => &[
        VerbDictionaryEntry { lemma: "dȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "dȯlbti" => &[
        VerbDictionaryEntry { lemma: "dȯlbti", addition: "(dȯlbe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dȯlgočasiti" => &[
        VerbDictionaryEntry { lemma: "dȯlgočasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dȯlgočasiti sę" => &[
        VerbDictionaryEntry { lemma: "dȯlgočasiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "dȯlžiti" => &[
        VerbDictionaryEntry { lemma: "dȯlžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "dȯžditi" => &[
        VerbDictionaryEntry { lemma: "dȯžditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "egzistovati" => &[
        VerbDictionaryEntry { lemma: "egzistovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ejakulovati" => &[
        VerbDictionaryEntry { lemma: "ejakulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "eksperimentovati" => &[
        VerbDictionaryEntry { lemma: "eksperimentovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "eksploatovati" => &[
        VerbDictionaryEntry { lemma: "eksploatovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "eksplodovati" => &[
        VerbDictionaryEntry { lemma: "eksplodovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "eksportovati" => &[
        VerbDictionaryEntry { lemma: "eksportovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ekstragovati" => &[
        VerbDictionaryEntry { lemma: "ekstragovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "eliminovati" => &[
        VerbDictionaryEntry { lemma: "eliminovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "emigrovati" => &[
        VerbDictionaryEntry { lemma: "emigrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "entuziazmovati" => &[
        VerbDictionaryEntry { lemma: "entuziazmovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "eskalovati" => &[
        VerbDictionaryEntry { lemma: "eskalovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "eskortovati" => &[
        VerbDictionaryEntry { lemma: "eskortovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "evakuovati" => &[
        VerbDictionaryEntry { lemma: "evakuovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "fabrikovati" => &[
        VerbDictionaryEntry { lemma: "fabrikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "falsifikovati" => &[
        VerbDictionaryEntry { lemma: "falsifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "farbovati" => &[
        VerbDictionaryEntry { lemma: "farbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "filmovati" => &[
        VerbDictionaryEntry { lemma: "filmovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "filtrovati" => &[
        VerbDictionaryEntry { lemma: "filtrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "finansovati" => &[
        VerbDictionaryEntry { lemma: "finansovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "flirtovati" => &[
        VerbDictionaryEntry { lemma: "flirtovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "formalizovati" => &[
        VerbDictionaryEntry { lemma: "formalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "formovati" => &[
        VerbDictionaryEntry { lemma: "formovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "formulovati" => &[
        VerbDictionaryEntry { lemma: "formulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "fotografovati" => &[
        VerbDictionaryEntry { lemma: "fotografovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "frustrovati" => &[
        VerbDictionaryEntry { lemma: "frustrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "funkcionovati" => &[
        VerbDictionaryEntry { lemma: "funkcionovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gadati" => &[
        VerbDictionaryEntry { lemma: "gadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "galopovati" => &[
        VerbDictionaryEntry { lemma: "galopovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "galvanizovati" => &[
        VerbDictionaryEntry { lemma: "galvanizovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "garantovati" => &[
        VerbDictionaryEntry { lemma: "garantovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "garnirovati" => &[
        VerbDictionaryEntry { lemma: "garnirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gasiti" => &[
        VerbDictionaryEntry { lemma: "gasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gasnųti" => &[
        VerbDictionaryEntry { lemma: "gasnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gazovati" => &[
        VerbDictionaryEntry { lemma: "gazovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "gazovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "generalizovati" => &[
        VerbDictionaryEntry { lemma: "generalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "generovati" => &[
        VerbDictionaryEntry { lemma: "generovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "gestikulovati" => &[
        VerbDictionaryEntry { lemma: "gestikulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gladiti" => &[
        VerbDictionaryEntry { lemma: "gladiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "globalizovati" => &[
        VerbDictionaryEntry { lemma: "globalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "glodati" => &[
        VerbDictionaryEntry { lemma: "glodati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gluhnųti" => &[
        VerbDictionaryEntry { lemma: "gluhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "glupěti" => &[
        VerbDictionaryEntry { lemma: "glupěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "glušiti" => &[
        VerbDictionaryEntry { lemma: "glušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "glådovati" => &[
        VerbDictionaryEntry { lemma: "glådovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "glåsiti" => &[
        VerbDictionaryEntry { lemma: "glåsiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "glåsovati" => &[
        VerbDictionaryEntry { lemma: "glåsovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ględati" => &[
        VerbDictionaryEntry { lemma: "ględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ględnųti" => &[
        VerbDictionaryEntry { lemma: "ględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ględěti" => &[
        VerbDictionaryEntry { lemma: "ględěti", addition: "(ględi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gnati" => &[
        VerbDictionaryEntry { lemma: "gnati", addition: "(gone)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gnesti" => &[
        VerbDictionaryEntry { lemma: "gnesti", addition: "(gnete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gniti" => &[
        VerbDictionaryEntry { lemma: "gniti", addition: "(gnije)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gnojiti" => &[
        VerbDictionaryEntry { lemma: "gnojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gnojiti sę" => &[
        VerbDictionaryEntry { lemma: "gnojiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "gněvati" => &[
        VerbDictionaryEntry { lemma: "gněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gněvati sę" => &[
        VerbDictionaryEntry { lemma: "gněvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "gnězditi sę" => &[
        VerbDictionaryEntry { lemma: "gnězditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "gnųti" => &[
        VerbDictionaryEntry { lemma: "gnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "goditi sę" => &[
        VerbDictionaryEntry { lemma: "goditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "godovati" => &[
        VerbDictionaryEntry { lemma: "godovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gojiti" => &[
        VerbDictionaryEntry { lemma: "gojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "goliti" => &[
        VerbDictionaryEntry { lemma: "goliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "goliti sę" => &[
        VerbDictionaryEntry { lemma: "goliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "gorčiti" => &[
        VerbDictionaryEntry { lemma: "gorčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gorěti" => &[
        VerbDictionaryEntry { lemma: "gorěti", addition: "(gori)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "goršiti" => &[
        VerbDictionaryEntry { lemma: "goršiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gostiti" => &[
        VerbDictionaryEntry { lemma: "gostiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gotoviti" => &[
        VerbDictionaryEntry { lemma: "gotoviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "govoriti" => &[
        VerbDictionaryEntry { lemma: "govoriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "goŕknųti" => &[
        VerbDictionaryEntry { lemma: "goŕknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "grabiti" => &[
        VerbDictionaryEntry { lemma: "grabiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "grabiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "graničiti" => &[
        VerbDictionaryEntry { lemma: "graničiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gravirovati" => &[
        VerbDictionaryEntry { lemma: "gravirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grbiti sę" => &[
        VerbDictionaryEntry { lemma: "grbiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "grditi" => &[
        VerbDictionaryEntry { lemma: "grditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grebti" => &[
        VerbDictionaryEntry { lemma: "grebti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grnųti" => &[
        VerbDictionaryEntry { lemma: "grnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grohtati" => &[
        VerbDictionaryEntry { lemma: "grohtati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gromaditi" => &[
        VerbDictionaryEntry { lemma: "gromaditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "groziti" => &[
        VerbDictionaryEntry { lemma: "groziti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "grupovati" => &[
        VerbDictionaryEntry { lemma: "grupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gryzati" => &[
        VerbDictionaryEntry { lemma: "gryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gryzti" => &[
        VerbDictionaryEntry { lemma: "gryzti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grėměti" => &[
        VerbDictionaryEntry { lemma: "grėměti", addition: "(grėmi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gręznųti" => &[
        VerbDictionaryEntry { lemma: "gręznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "grěti" => &[
        VerbDictionaryEntry { lemma: "grěti", addition: "(grěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "grěšiti" => &[
        VerbDictionaryEntry { lemma: "grěšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gubiti" => &[
        VerbDictionaryEntry { lemma: "gubiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gubiti sę" => &[
        VerbDictionaryEntry { lemma: "gubiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "gybati" => &[
        VerbDictionaryEntry { lemma: "gybati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gybnųti" => &[
        VerbDictionaryEntry { lemma: "gybnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "gųstiti" => &[
        VerbDictionaryEntry { lemma: "gųstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gųstěti" => &[
        VerbDictionaryEntry { lemma: "gųstěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "gȯltati" => &[
        VerbDictionaryEntry { lemma: "gȯltati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "gȯltnųti" => &[
        VerbDictionaryEntry { lemma: "gȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "halucinovati" => &[
        VerbDictionaryEntry { lemma: "halucinovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "harakterizovati" => &[
        VerbDictionaryEntry { lemma: "harakterizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "harmonizovati" => &[
        VerbDictionaryEntry { lemma: "harmonizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hihotati sę" => &[
        VerbDictionaryEntry { lemma: "hihotati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hipnotizovati" => &[
        VerbDictionaryEntry { lemma: "hipnotizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hlipati" => &[
        VerbDictionaryEntry { lemma: "hlipati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hlipnųti" => &[
        VerbDictionaryEntry { lemma: "hlipnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "hlåditi" => &[
        VerbDictionaryEntry { lemma: "hlåditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hlåditi sę" => &[
        VerbDictionaryEntry { lemma: "hlåditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hlåděti" => &[
        VerbDictionaryEntry { lemma: "hlåděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hmuriti" => &[
        VerbDictionaryEntry { lemma: "hmuriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hmuriti sę" => &[
        VerbDictionaryEntry { lemma: "hmuriti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hoditi" => &[
        VerbDictionaryEntry { lemma: "hoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "homogenizovati" => &[
        VerbDictionaryEntry { lemma: "homogenizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hotěti" => &[
        VerbDictionaryEntry { lemma: "hotěti", addition: "(hoće)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hovati" => &[
        VerbDictionaryEntry { lemma: "hovati", addition: "(hovaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hrapati" => &[
        VerbDictionaryEntry { lemma: "hrapati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hrapnųti" => &[
        VerbDictionaryEntry { lemma: "hrapnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "hromati" => &[
        VerbDictionaryEntry { lemma: "hromati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hroměti" => &[
        VerbDictionaryEntry { lemma: "hroměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hrupati" => &[
        VerbDictionaryEntry { lemma: "hrupati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hrustati" => &[
        VerbDictionaryEntry { lemma: "hrustati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hråniti" => &[
        VerbDictionaryEntry { lemma: "hråniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hråniti sę" => &[
        VerbDictionaryEntry { lemma: "hråniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hrčati" => &[
        VerbDictionaryEntry { lemma: "hrčati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "htěti" => &[
        VerbDictionaryEntry { lemma: "htěti", addition: "(hće)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hudnųti" => &[
        VerbDictionaryEntry { lemma: "hudnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hvaliti" => &[
        VerbDictionaryEntry { lemma: "hvaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hvastati sę" => &[
        VerbDictionaryEntry { lemma: "hvastati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hvatati" => &[
        VerbDictionaryEntry { lemma: "hvatati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hvorěti" => &[
        VerbDictionaryEntry { lemma: "hvorěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "hvějati" => &[
        VerbDictionaryEntry { lemma: "hvějati", addition: "(hvěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "hvějati sę" => &[
        VerbDictionaryEntry { lemma: "hvějati sę", addition: "(hvěje)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "hybiti" => &[
        VerbDictionaryEntry { lemma: "hybiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "idealizovati" => &[
        VerbDictionaryEntry { lemma: "idealizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "identifikovati" => &[
        VerbDictionaryEntry { lemma: "identifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "idti" => &[
        VerbDictionaryEntry { lemma: "idti", addition: "(ide; šėl)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ignorovati" => &[
        VerbDictionaryEntry { lemma: "ignorovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "igrati" => &[
        VerbDictionaryEntry { lemma: "igrati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ikati" => &[
        VerbDictionaryEntry { lemma: "ikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ilustrovati" => &[
        VerbDictionaryEntry { lemma: "ilustrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "imati" => &[
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "imati", addition: "(imaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "imenovati" => &[
        VerbDictionaryEntry { lemma: "imenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "imigrovati" => &[
        VerbDictionaryEntry { lemma: "imigrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "imitovati" => &[
        VerbDictionaryEntry { lemma: "imitovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "implantovati" => &[
        VerbDictionaryEntry { lemma: "implantovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "imponovati" => &[
        VerbDictionaryEntry { lemma: "imponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "importovati" => &[
        VerbDictionaryEntry { lemma: "importovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "improvizovati" => &[
        VerbDictionaryEntry { lemma: "improvizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "iměti" => &[
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "iměti", addition: "(imaje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "industrializovati" => &[
        VerbDictionaryEntry { lemma: "industrializovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "informovati" => &[
        VerbDictionaryEntry { lemma: "informovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "inicializovati" => &[
        VerbDictionaryEntry { lemma: "inicializovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "inspirovati" => &[
        VerbDictionaryEntry { lemma: "inspirovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "instalovati" => &[
        VerbDictionaryEntry { lemma: "instalovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "integrovati" => &[
        VerbDictionaryEntry { lemma: "integrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "interesovati" => &[
        VerbDictionaryEntry { lemma: "interesovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "interesovati sę" => &[
        VerbDictionaryEntry { lemma: "interesovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "internacionalizovati" => &[
        VerbDictionaryEntry { lemma: "internacionalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "interpretovati" => &[
        VerbDictionaryEntry { lemma: "interpretovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "intrigovati" => &[
        VerbDictionaryEntry { lemma: "intrigovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "investovati" => &[
        VerbDictionaryEntry { lemma: "investovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ironizovati" => &[
        VerbDictionaryEntry { lemma: "ironizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "iskati" => &[
        VerbDictionaryEntry { lemma: "iskati", addition: "(išče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iskriti" => &[
        VerbDictionaryEntry { lemma: "iskriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "istnovati" => &[
        VerbDictionaryEntry { lemma: "istnovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izbaviti" => &[
        VerbDictionaryEntry { lemma: "izbaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "izbaviti", addition: "(+2)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: Some(2) },
    ],
    "izbaviti sę" => &[
        VerbDictionaryEntry { lemma: "izbaviti sę", addition: "(+2)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: Some(2) },
    ],
    "izbavjati" => &[
        VerbDictionaryEntry { lemma: "izbavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "izbavjati", addition: "(+2)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(2) },
    ],
    "izbavjati sę" => &[
        VerbDictionaryEntry { lemma: "izbavjati sę", addition: "(+2)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(2) },
    ],
    "izbirati" => &[
        VerbDictionaryEntry { lemma: "izbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izbiti" => &[
        VerbDictionaryEntry { lemma: "izbiti", addition: "(izbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izbivati" => &[
        VerbDictionaryEntry { lemma: "izbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izbljuvati" => &[
        VerbDictionaryEntry { lemma: "izbljuvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izbombardovati" => &[
        VerbDictionaryEntry { lemma: "izbombardovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izbrati" => &[
        VerbDictionaryEntry { lemma: "izbrati", addition: "(izbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izbudovati" => &[
        VerbDictionaryEntry { lemma: "izbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izbuhati" => &[
        VerbDictionaryEntry { lemma: "izbuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izbuhnųti" => &[
        VerbDictionaryEntry { lemma: "izbuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izběgati" => &[
        VerbDictionaryEntry { lemma: "izběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "izběgati", addition: "(+2)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(2) },
    ],
    "izběgti" => &[
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži) (+2)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: Some(2) },
        VerbDictionaryEntry { lemma: "izběgti", addition: "(izběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izcěliti" => &[
        VerbDictionaryEntry { lemma: "izcěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izcěliti sę" => &[
        VerbDictionaryEntry { lemma: "izcěliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izcěljati" => &[
        VerbDictionaryEntry { lemma: "izcěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izcěljati sę" => &[
        VerbDictionaryEntry { lemma: "izcěljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izdati" => &[
        VerbDictionaryEntry { lemma: "izdati", addition: "(izda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izdavati" => &[
        VerbDictionaryEntry { lemma: "izdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izdavati sę" => &[
        VerbDictionaryEntry { lemma: "izdavati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izdojiti" => &[
        VerbDictionaryEntry { lemma: "izdojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izdumati" => &[
        VerbDictionaryEntry { lemma: "izdumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izdȯlbti" => &[
        VerbDictionaryEntry { lemma: "izdȯlbti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izganjati" => &[
        VerbDictionaryEntry { lemma: "izganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izgladiti" => &[
        VerbDictionaryEntry { lemma: "izgladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izglašati" => &[
        VerbDictionaryEntry { lemma: "izglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izglåsiti" => &[
        VerbDictionaryEntry { lemma: "izglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izględati" => &[
        VerbDictionaryEntry { lemma: "izględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izględnųti" => &[
        VerbDictionaryEntry { lemma: "izględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izgnati" => &[
        VerbDictionaryEntry { lemma: "izgnati", addition: "(izgone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izgniti" => &[
        VerbDictionaryEntry { lemma: "izgniti", addition: "(izgnije)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izgojiti" => &[
        VerbDictionaryEntry { lemma: "izgojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izgorěti" => &[
        VerbDictionaryEntry { lemma: "izgorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izgovarjati" => &[
        VerbDictionaryEntry { lemma: "izgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izgovoriti" => &[
        VerbDictionaryEntry { lemma: "izgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izgubiti" => &[
        VerbDictionaryEntry { lemma: "izgubiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izgubiti sę" => &[
        VerbDictionaryEntry { lemma: "izgubiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izgynųti" => &[
        VerbDictionaryEntry { lemma: "izgynųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izhoditi" => &[
        VerbDictionaryEntry { lemma: "izhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izigrati" => &[
        VerbDictionaryEntry { lemma: "izigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izigryvati" => &[
        VerbDictionaryEntry { lemma: "izigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izimati" => &[
        VerbDictionaryEntry { lemma: "izimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iziskati" => &[
        VerbDictionaryEntry { lemma: "iziskati", addition: "(izišče)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iziskyvati" => &[
        VerbDictionaryEntry { lemma: "iziskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izjasniti" => &[
        VerbDictionaryEntry { lemma: "izjasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izjasnjati" => &[
        VerbDictionaryEntry { lemma: "izjasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izjaviti" => &[
        VerbDictionaryEntry { lemma: "izjaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izjaviti sę" => &[
        VerbDictionaryEntry { lemma: "izjaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izjavjati" => &[
        VerbDictionaryEntry { lemma: "izjavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izjavjati sę" => &[
        VerbDictionaryEntry { lemma: "izjavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izjehati" => &[
        VerbDictionaryEntry { lemma: "izjehati", addition: "(izjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izježđati" => &[
        VerbDictionaryEntry { lemma: "izježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izjęti" => &[
        VerbDictionaryEntry { lemma: "izjęti", addition: "(izȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkalkulovati" => &[
        VerbDictionaryEntry { lemma: "izkalkulovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkladati" => &[
        VerbDictionaryEntry { lemma: "izkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izključati" => &[
        VerbDictionaryEntry { lemma: "izključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izključiti" => &[
        VerbDictionaryEntry { lemma: "izključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkopati" => &[
        VerbDictionaryEntry { lemma: "izkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkopyvati" => &[
        VerbDictionaryEntry { lemma: "izkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkoreniti" => &[
        VerbDictionaryEntry { lemma: "izkoreniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkorenjati" => &[
        VerbDictionaryEntry { lemma: "izkorenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkoristati" => &[
        VerbDictionaryEntry { lemma: "izkoristati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkoristiti" => &[
        VerbDictionaryEntry { lemma: "izkoristiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkoristyvati" => &[
        VerbDictionaryEntry { lemma: "izkoristyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkositi" => &[
        VerbDictionaryEntry { lemma: "izkositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkovati" => &[
        VerbDictionaryEntry { lemma: "izkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkovyvati" => &[
        VerbDictionaryEntry { lemma: "izkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkrikati" => &[
        VerbDictionaryEntry { lemma: "izkrikati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkriknųti" => &[
        VerbDictionaryEntry { lemma: "izkriknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkrviti" => &[
        VerbDictionaryEntry { lemma: "izkrviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkupiti" => &[
        VerbDictionaryEntry { lemma: "izkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkusiti" => &[
        VerbDictionaryEntry { lemma: "izkusiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkušati" => &[
        VerbDictionaryEntry { lemma: "izkušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkydati" => &[
        VerbDictionaryEntry { lemma: "izkydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izkydnųti" => &[
        VerbDictionaryEntry { lemma: "izkydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izkųpati" => &[
        VerbDictionaryEntry { lemma: "izkųpati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izlagati" => &[
        VerbDictionaryEntry { lemma: "izlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izliti" => &[
        VerbDictionaryEntry { lemma: "izliti", addition: "(izlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izlivati" => &[
        VerbDictionaryEntry { lemma: "izlivati", addition: "(izlije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izlomiti" => &[
        VerbDictionaryEntry { lemma: "izlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izloviti" => &[
        VerbDictionaryEntry { lemma: "izloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izložiti" => &[
        VerbDictionaryEntry { lemma: "izložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izlęgti" => &[
        VerbDictionaryEntry { lemma: "izlęgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izlęgti sę" => &[
        VerbDictionaryEntry { lemma: "izlęgti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izlězati" => &[
        VerbDictionaryEntry { lemma: "izlězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izlězti" => &[
        VerbDictionaryEntry { lemma: "izlězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izlěčiti" => &[
        VerbDictionaryEntry { lemma: "izlěčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izlěčiti sę" => &[
        VerbDictionaryEntry { lemma: "izlěčiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izmagati" => &[
        VerbDictionaryEntry { lemma: "izmagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izmamiti" => &[
        VerbDictionaryEntry { lemma: "izmamiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmamjati" => &[
        VerbDictionaryEntry { lemma: "izmamjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izmesti" => &[
        VerbDictionaryEntry { lemma: "izmesti", addition: "(izmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmirati" => &[
        VerbDictionaryEntry { lemma: "izmirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izmodelovati" => &[
        VerbDictionaryEntry { lemma: "izmodelovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmogti" => &[
        VerbDictionaryEntry { lemma: "izmogti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmoriti" => &[
        VerbDictionaryEntry { lemma: "izmoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmočiti" => &[
        VerbDictionaryEntry { lemma: "izmočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izmrěti" => &[
        VerbDictionaryEntry { lemma: "izmrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izmysliti" => &[
        VerbDictionaryEntry { lemma: "izmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmysljati" => &[
        VerbDictionaryEntry { lemma: "izmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izměniti" => &[
        VerbDictionaryEntry { lemma: "izměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izměnjati" => &[
        VerbDictionaryEntry { lemma: "izměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izměriti" => &[
        VerbDictionaryEntry { lemma: "izměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izměstiti" => &[
        VerbDictionaryEntry { lemma: "izměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izmětati" => &[
        VerbDictionaryEntry { lemma: "izmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izměšćati" => &[
        VerbDictionaryEntry { lemma: "izměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iznahoditi" => &[
        VerbDictionaryEntry { lemma: "iznahoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iznajdti" => &[
        VerbDictionaryEntry { lemma: "iznajdti", addition: "(iznajde; iznašėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iznajmati" => &[
        VerbDictionaryEntry { lemma: "iznajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iznajęti" => &[
        VerbDictionaryEntry { lemma: "iznajęti", addition: "(iznajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iznasilovati" => &[
        VerbDictionaryEntry { lemma: "iznasilovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iznemogti" => &[
        VerbDictionaryEntry { lemma: "iznemogti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "iznesti" => &[
        VerbDictionaryEntry { lemma: "iznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izniknųti" => &[
        VerbDictionaryEntry { lemma: "izniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izniščiti" => &[
        VerbDictionaryEntry { lemma: "izniščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iznositi" => &[
        VerbDictionaryEntry { lemma: "iznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iznuditi" => &[
        VerbDictionaryEntry { lemma: "iznuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iznuriti" => &[
        VerbDictionaryEntry { lemma: "iznuriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "iznurjati" => &[
        VerbDictionaryEntry { lemma: "iznurjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "iznuđati" => &[
        VerbDictionaryEntry { lemma: "iznuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izobličiti" => &[
        VerbDictionaryEntry { lemma: "izobličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izobraziti" => &[
        VerbDictionaryEntry { lemma: "izobraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izobražati" => &[
        VerbDictionaryEntry { lemma: "izobražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izolovati" => &[
        VerbDictionaryEntry { lemma: "izolovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izopačati" => &[
        VerbDictionaryEntry { lemma: "izopačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izopačiti" => &[
        VerbDictionaryEntry { lemma: "izopačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izorati" => &[
        VerbDictionaryEntry { lemma: "izorati", addition: "(izoŕe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpadati" => &[
        VerbDictionaryEntry { lemma: "izpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izpasti" => &[
        VerbDictionaryEntry { lemma: "izpasti", addition: "(izpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izpekti" => &[
        VerbDictionaryEntry { lemma: "izpekti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpepeliti" => &[
        VerbDictionaryEntry { lemma: "izpepeliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izphati" => &[
        VerbDictionaryEntry { lemma: "izphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpihati" => &[
        VerbDictionaryEntry { lemma: "izpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpiti" => &[
        VerbDictionaryEntry { lemma: "izpiti", addition: "(izpije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izplatiti" => &[
        VerbDictionaryEntry { lemma: "izplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izplaćati" => &[
        VerbDictionaryEntry { lemma: "izplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpljunųti" => &[
        VerbDictionaryEntry { lemma: "izpljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpljuvati" => &[
        VerbDictionaryEntry { lemma: "izpljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpluti" => &[
        VerbDictionaryEntry { lemma: "izpluti", addition: "(izplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izplyvati" => &[
        VerbDictionaryEntry { lemma: "izplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izplyvti" => &[
        VerbDictionaryEntry { lemma: "izplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izplåšiti" => &[
        VerbDictionaryEntry { lemma: "izplåšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izplěti" => &[
        VerbDictionaryEntry { lemma: "izplěti", addition: "(izplěje/izplěve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpovědati" => &[
        VerbDictionaryEntry { lemma: "izpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpověděti" => &[
        VerbDictionaryEntry { lemma: "izpověděti", addition: "(izpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izprati" => &[
        VerbDictionaryEntry { lemma: "izprati", addition: "(izpere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpraviti" => &[
        VerbDictionaryEntry { lemma: "izpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpravjati" => &[
        VerbDictionaryEntry { lemma: "izpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izprašati" => &[
        VerbDictionaryEntry { lemma: "izprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izprobovati" => &[
        VerbDictionaryEntry { lemma: "izprobovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izprobovyvati" => &[
        VerbDictionaryEntry { lemma: "izprobovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izprositi" => &[
        VerbDictionaryEntry { lemma: "izprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpråzdniti" => &[
        VerbDictionaryEntry { lemma: "izpråzdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpråzdnjati" => &[
        VerbDictionaryEntry { lemma: "izpråzdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpręgati" => &[
        VerbDictionaryEntry { lemma: "izpręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpręgti" => &[
        VerbDictionaryEntry { lemma: "izpręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpustiti" => &[
        VerbDictionaryEntry { lemma: "izpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpušćati" => &[
        VerbDictionaryEntry { lemma: "izpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpytati" => &[
        VerbDictionaryEntry { lemma: "izpytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpytyvati" => &[
        VerbDictionaryEntry { lemma: "izpytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpųditi" => &[
        VerbDictionaryEntry { lemma: "izpųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpȯlniti" => &[
        VerbDictionaryEntry { lemma: "izpȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izpȯlnjati" => &[
        VerbDictionaryEntry { lemma: "izpȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izpȯlzati" => &[
        VerbDictionaryEntry { lemma: "izpȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izpȯlzti" => &[
        VerbDictionaryEntry { lemma: "izpȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izravnjati" => &[
        VerbDictionaryEntry { lemma: "izravnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izravnjati sę" => &[
        VerbDictionaryEntry { lemma: "izravnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izraziti" => &[
        VerbDictionaryEntry { lemma: "izraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izraziti sę" => &[
        VerbDictionaryEntry { lemma: "izraziti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izrađati sę" => &[
        VerbDictionaryEntry { lemma: "izrađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izražati" => &[
        VerbDictionaryEntry { lemma: "izražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izražati sę" => &[
        VerbDictionaryEntry { lemma: "izražati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izroditi sę" => &[
        VerbDictionaryEntry { lemma: "izroditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izrvati" => &[
        VerbDictionaryEntry { lemma: "izrvati", addition: "(izrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izryvati" => &[
        VerbDictionaryEntry { lemma: "izryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izråbiti" => &[
        VerbDictionaryEntry { lemma: "izråbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izråbotati" => &[
        VerbDictionaryEntry { lemma: "izråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izråbotyvati" => &[
        VerbDictionaryEntry { lemma: "izråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izråsti" => &[
        VerbDictionaryEntry { lemma: "izråsti", addition: "(izråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izråvniti" => &[
        VerbDictionaryEntry { lemma: "izråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izråvniti sę" => &[
        VerbDictionaryEntry { lemma: "izråvniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izrěkati" => &[
        VerbDictionaryEntry { lemma: "izrěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izrěkti" => &[
        VerbDictionaryEntry { lemma: "izrěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izrězati" => &[
        VerbDictionaryEntry { lemma: "izrězati", addition: "(izrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izrězyvati" => &[
        VerbDictionaryEntry { lemma: "izrězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izrųbati" => &[
        VerbDictionaryEntry { lemma: "izrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izrųbyvati" => &[
        VerbDictionaryEntry { lemma: "izrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izseliti" => &[
        VerbDictionaryEntry { lemma: "izseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izseliti sę" => &[
        VerbDictionaryEntry { lemma: "izseliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izseljati" => &[
        VerbDictionaryEntry { lemma: "izseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izseljati sę" => &[
        VerbDictionaryEntry { lemma: "izseljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izslati" => &[
        VerbDictionaryEntry { lemma: "izslati", addition: "(izšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izslavjati" => &[
        VerbDictionaryEntry { lemma: "izslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izslavjati sę" => &[
        VerbDictionaryEntry { lemma: "izslavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izsloviti" => &[
        VerbDictionaryEntry { lemma: "izsloviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsloviti sę" => &[
        VerbDictionaryEntry { lemma: "izsloviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izslušati" => &[
        VerbDictionaryEntry { lemma: "izslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izslušivati" => &[
        VerbDictionaryEntry { lemma: "izslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izslědovati" => &[
        VerbDictionaryEntry { lemma: "izslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsmrkati sę" => &[
        VerbDictionaryEntry { lemma: "izsmrkati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izsmějati" => &[
        VerbDictionaryEntry { lemma: "izsmějati", addition: "(izsměje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsmějivati" => &[
        VerbDictionaryEntry { lemma: "izsmějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izspati sę" => &[
        VerbDictionaryEntry { lemma: "izspati sę", addition: "(izspi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izstaviti" => &[
        VerbDictionaryEntry { lemma: "izstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izstavjati" => &[
        VerbDictionaryEntry { lemma: "izstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izstrěliti" => &[
        VerbDictionaryEntry { lemma: "izstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izstųpati" => &[
        VerbDictionaryEntry { lemma: "izstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izstųpiti" => &[
        VerbDictionaryEntry { lemma: "izstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izsunųti" => &[
        VerbDictionaryEntry { lemma: "izsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsuvati" => &[
        VerbDictionaryEntry { lemma: "izsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsušati" => &[
        VerbDictionaryEntry { lemma: "izsušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsušiti" => &[
        VerbDictionaryEntry { lemma: "izsušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsyhati" => &[
        VerbDictionaryEntry { lemma: "izsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izsylati" => &[
        VerbDictionaryEntry { lemma: "izsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsypati" => &[
        VerbDictionaryEntry { lemma: "izsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsysati" => &[
        VerbDictionaryEntry { lemma: "izsysati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsěkati" => &[
        VerbDictionaryEntry { lemma: "izsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izsěkti" => &[
        VerbDictionaryEntry { lemma: "izsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "izsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izsȯsati" => &[
        VerbDictionaryEntry { lemma: "izsȯsati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztekti" => &[
        VerbDictionaryEntry { lemma: "iztekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "iztirati" => &[
        VerbDictionaryEntry { lemma: "iztirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iztkati" => &[
        VerbDictionaryEntry { lemma: "iztkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztratiti" => &[
        VerbDictionaryEntry { lemma: "iztratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztrgati" => &[
        VerbDictionaryEntry { lemma: "iztrgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iztrgnųti" => &[
        VerbDictionaryEntry { lemma: "iztrgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztręsati" => &[
        VerbDictionaryEntry { lemma: "iztręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iztręsti" => &[
        VerbDictionaryEntry { lemma: "iztręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztrěti" => &[
        VerbDictionaryEntry { lemma: "iztrěti", addition: "(iztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztvarjati" => &[
        VerbDictionaryEntry { lemma: "iztvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iztvoriti" => &[
        VerbDictionaryEntry { lemma: "iztvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztęgati" => &[
        VerbDictionaryEntry { lemma: "iztęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "iztęgnųti" => &[
        VerbDictionaryEntry { lemma: "iztęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztěkati" => &[
        VerbDictionaryEntry { lemma: "iztěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "iztŕpěti" => &[
        VerbDictionaryEntry { lemma: "iztŕpěti", addition: "(iztŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztŕti" => &[
        VerbDictionaryEntry { lemma: "iztŕti", addition: "(iztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "iztȯlkti" => &[
        VerbDictionaryEntry { lemma: "iztȯlkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izumirati" => &[
        VerbDictionaryEntry { lemma: "izumirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izumrěti" => &[
        VerbDictionaryEntry { lemma: "izumrěti", addition: "(izumre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izumŕti" => &[
        VerbDictionaryEntry { lemma: "izumŕti", addition: "(izumre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izučati" => &[
        VerbDictionaryEntry { lemma: "izučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izučiti" => &[
        VerbDictionaryEntry { lemma: "izučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvajati" => &[
        VerbDictionaryEntry { lemma: "izvajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvaljnjati" => &[
        VerbDictionaryEntry { lemma: "izvaljnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvesti" => &[
        VerbDictionaryEntry { lemma: "izvesti", addition: "(izvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvezti" => &[
        VerbDictionaryEntry { lemma: "izvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvihnųti" => &[
        VerbDictionaryEntry { lemma: "izvihnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izviniti" => &[
        VerbDictionaryEntry { lemma: "izviniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izviniti sę" => &[
        VerbDictionaryEntry { lemma: "izviniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "izvinjati" => &[
        VerbDictionaryEntry { lemma: "izvinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvinjati sę" => &[
        VerbDictionaryEntry { lemma: "izvinjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "izvlastniti" => &[
        VerbDictionaryEntry { lemma: "izvlastniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvlastnjati" => &[
        VerbDictionaryEntry { lemma: "izvlastnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvlěkati" => &[
        VerbDictionaryEntry { lemma: "izvlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvlěkti" => &[
        VerbDictionaryEntry { lemma: "izvlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvoditi" => &[
        VerbDictionaryEntry { lemma: "izvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvoliti" => &[
        VerbDictionaryEntry { lemma: "izvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvoljniti" => &[
        VerbDictionaryEntry { lemma: "izvoljniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvoziti" => &[
        VerbDictionaryEntry { lemma: "izvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvraćati" => &[
        VerbDictionaryEntry { lemma: "izvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izvråtiti" => &[
        VerbDictionaryEntry { lemma: "izvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvěstiti" => &[
        VerbDictionaryEntry { lemma: "izvěstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izvěšćati" => &[
        VerbDictionaryEntry { lemma: "izvěšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izznačati" => &[
        VerbDictionaryEntry { lemma: "izznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izznačiti" => &[
        VerbDictionaryEntry { lemma: "izznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izzvati" => &[
        VerbDictionaryEntry { lemma: "izzvati", addition: "(izzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izzyvati" => &[
        VerbDictionaryEntry { lemma: "izzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izčezati" => &[
        VerbDictionaryEntry { lemma: "izčezati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "izčeznųti" => &[
        VerbDictionaryEntry { lemma: "izčeznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "izčisliti" => &[
        VerbDictionaryEntry { lemma: "izčisliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izčisljati" => &[
        VerbDictionaryEntry { lemma: "izčisljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izčistiti" => &[
        VerbDictionaryEntry { lemma: "izčistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izčrkati" => &[
        VerbDictionaryEntry { lemma: "izčrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izčrknųti" => &[
        VerbDictionaryEntry { lemma: "izčrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izčrpati" => &[
        VerbDictionaryEntry { lemma: "izčrpati", addition: "(izčrpe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izčrpyvati" => &[
        VerbDictionaryEntry { lemma: "izčrpyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izšiti" => &[
        VerbDictionaryEntry { lemma: "izšiti", addition: "(izšije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izšivati" => &[
        VerbDictionaryEntry { lemma: "izšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "izškoliti" => &[
        VerbDictionaryEntry { lemma: "izškoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izžęti" => &[
        VerbDictionaryEntry { lemma: "izžęti", addition: "(izžne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "izȯjdti" => &[
        VerbDictionaryEntry { lemma: "izȯjdti", addition: "(izȯjde; izšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "jagniti sę" => &[
        VerbDictionaryEntry { lemma: "jagniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "jalověti" => &[
        VerbDictionaryEntry { lemma: "jalověti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "jasněti" => &[
        VerbDictionaryEntry { lemma: "jasněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "javiti sę" => &[
        VerbDictionaryEntry { lemma: "javiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "javjati sę" => &[
        VerbDictionaryEntry { lemma: "javjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "je" => &[
        VerbDictionaryEntry { lemma: "je", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "jebati" => &[
        VerbDictionaryEntry { lemma: "jebati", addition: "(jebe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "jedati" => &[
        VerbDictionaryEntry { lemma: "jedati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "jehati" => &[
        VerbDictionaryEntry { lemma: "jehati", addition: "(jede)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "jest" => &[
        VerbDictionaryEntry { lemma: "jest", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "jesti" => &[
        VerbDictionaryEntry { lemma: "jesti", addition: "(je)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "jestvovati" => &[
        VerbDictionaryEntry { lemma: "jestvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "jezditi" => &[
        VerbDictionaryEntry { lemma: "jezditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ježiti" => &[
        VerbDictionaryEntry { lemma: "ježiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ježiti sę" => &[
        VerbDictionaryEntry { lemma: "ježiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "jęti" => &[
        VerbDictionaryEntry { lemma: "jęti", addition: "(jme)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "jęčati" => &[
        VerbDictionaryEntry { lemma: "jęčati", addition: "(jęči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kaditi" => &[
        VerbDictionaryEntry { lemma: "kaditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kajati sę" => &[
        VerbDictionaryEntry { lemma: "kajati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kakati" => &[
        VerbDictionaryEntry { lemma: "kakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kaliti" => &[
        VerbDictionaryEntry { lemma: "kaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kalkulovati" => &[
        VerbDictionaryEntry { lemma: "kalkulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kamenovati" => &[
        VerbDictionaryEntry { lemma: "kamenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kameněti" => &[
        VerbDictionaryEntry { lemma: "kameněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kanalizovati" => &[
        VerbDictionaryEntry { lemma: "kanalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kandidovati" => &[
        VerbDictionaryEntry { lemma: "kandidovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kanonizovati" => &[
        VerbDictionaryEntry { lemma: "kanonizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kapati" => &[
        VerbDictionaryEntry { lemma: "kapati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kapitulovati" => &[
        VerbDictionaryEntry { lemma: "kapitulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kapnųti" => &[
        VerbDictionaryEntry { lemma: "kapnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "kaprizovati" => &[
        VerbDictionaryEntry { lemma: "kaprizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "karati" => &[
        VerbDictionaryEntry { lemma: "karati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kastrovati" => &[
        VerbDictionaryEntry { lemma: "kastrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kategorizovati" => &[
        VerbDictionaryEntry { lemma: "kategorizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kazati" => &[
        VerbDictionaryEntry { lemma: "kazati", addition: "(kaže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kazati sę" => &[
        VerbDictionaryEntry { lemma: "kazati sę", addition: "(kaže)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kaziti" => &[
        VerbDictionaryEntry { lemma: "kaziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kazniti" => &[
        VerbDictionaryEntry { lemma: "kazniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kašljati" => &[
        VerbDictionaryEntry { lemma: "kašljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "klanjati sę" => &[
        VerbDictionaryEntry { lemma: "klanjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "klasifikovati" => &[
        VerbDictionaryEntry { lemma: "klasifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klasti" => &[
        VerbDictionaryEntry { lemma: "klasti", addition: "(klade)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klejiti" => &[
        VerbDictionaryEntry { lemma: "klejiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klevetati" => &[
        VerbDictionaryEntry { lemma: "klevetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "klicati" => &[
        VerbDictionaryEntry { lemma: "klicati", addition: "(kliče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klikati" => &[
        VerbDictionaryEntry { lemma: "klikati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kliknųti" => &[
        VerbDictionaryEntry { lemma: "kliknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kljunųti" => &[
        VerbDictionaryEntry { lemma: "kljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kljusati" => &[
        VerbDictionaryEntry { lemma: "kljusati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kljuvati" => &[
        VerbDictionaryEntry { lemma: "kljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klokotati" => &[
        VerbDictionaryEntry { lemma: "klokotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kloniti" => &[
        VerbDictionaryEntry { lemma: "kloniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kloniti sę" => &[
        VerbDictionaryEntry { lemma: "kloniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "klåsiti sę" => &[
        VerbDictionaryEntry { lemma: "klåsiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "klåti" => &[
        VerbDictionaryEntry { lemma: "klåti", addition: "(kolje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klåtiti" => &[
        VerbDictionaryEntry { lemma: "klåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "klåtiti sę" => &[
        VerbDictionaryEntry { lemma: "klåtiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "klęknųti" => &[
        VerbDictionaryEntry { lemma: "klęknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "klęti" => &[
        VerbDictionaryEntry { lemma: "klęti", addition: "(klne)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "klęčati" => &[
        VerbDictionaryEntry { lemma: "klęčati", addition: "(klęče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "klěskati" => &[
        VerbDictionaryEntry { lemma: "klěskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kodifikovati" => &[
        VerbDictionaryEntry { lemma: "kodifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kodovati" => &[
        VerbDictionaryEntry { lemma: "kodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koketovati" => &[
        VerbDictionaryEntry { lemma: "koketovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kokodakati" => &[
        VerbDictionaryEntry { lemma: "kokodakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kolonizovati" => &[
        VerbDictionaryEntry { lemma: "kolonizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kolorovati" => &[
        VerbDictionaryEntry { lemma: "kolorovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kolovati" => &[
        VerbDictionaryEntry { lemma: "kolovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kolędovati" => &[
        VerbDictionaryEntry { lemma: "kolędovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kolěbati" => &[
        VerbDictionaryEntry { lemma: "kolěbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kolěbati sę" => &[
        VerbDictionaryEntry { lemma: "kolěbati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kombinovati" => &[
        VerbDictionaryEntry { lemma: "kombinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "komentovati" => &[
        VerbDictionaryEntry { lemma: "komentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kompensovati" => &[
        VerbDictionaryEntry { lemma: "kompensovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kompjuterizovati" => &[
        VerbDictionaryEntry { lemma: "kompjuterizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "komplikovati" => &[
        VerbDictionaryEntry { lemma: "komplikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "komponovati" => &[
        VerbDictionaryEntry { lemma: "komponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "komunikovati" => &[
        VerbDictionaryEntry { lemma: "komunikovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "koncentrovati" => &[
        VerbDictionaryEntry { lemma: "koncentrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kondensovati sę" => &[
        VerbDictionaryEntry { lemma: "kondensovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "konfiskovati" => &[
        VerbDictionaryEntry { lemma: "konfiskovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "konfliktovati" => &[
        VerbDictionaryEntry { lemma: "konfliktovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "konkurovati" => &[
        VerbDictionaryEntry { lemma: "konkurovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "konservovati" => &[
        VerbDictionaryEntry { lemma: "konservovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "konstruovati" => &[
        VerbDictionaryEntry { lemma: "konstruovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kontrabandovati" => &[
        VerbDictionaryEntry { lemma: "kontrabandovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kontrolovati" => &[
        VerbDictionaryEntry { lemma: "kontrolovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "končati" => &[
        VerbDictionaryEntry { lemma: "končati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "končati sę" => &[
        VerbDictionaryEntry { lemma: "končati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kooperovati" => &[
        VerbDictionaryEntry { lemma: "kooperovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "koordinovati" => &[
        VerbDictionaryEntry { lemma: "koordinovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kopati" => &[
        VerbDictionaryEntry { lemma: "kopati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kopijovati" => &[
        VerbDictionaryEntry { lemma: "kopijovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kopirovati" => &[
        VerbDictionaryEntry { lemma: "kopirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "korelovati" => &[
        VerbDictionaryEntry { lemma: "korelovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "koreniti sę" => &[
        VerbDictionaryEntry { lemma: "koreniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "korigovati" => &[
        VerbDictionaryEntry { lemma: "korigovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koristati" => &[
        VerbDictionaryEntry { lemma: "koristati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koristiti" => &[
        VerbDictionaryEntry { lemma: "koristiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koriti" => &[
        VerbDictionaryEntry { lemma: "koriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koronovati" => &[
        VerbDictionaryEntry { lemma: "koronovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kositi" => &[
        VerbDictionaryEntry { lemma: "kositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kotiti sę" => &[
        VerbDictionaryEntry { lemma: "kotiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kovati" => &[
        VerbDictionaryEntry { lemma: "kovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "koštovati" => &[
        VerbDictionaryEntry { lemma: "koštovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krakati" => &[
        VerbDictionaryEntry { lemma: "krakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "krasiti" => &[
        VerbDictionaryEntry { lemma: "krasiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krasti" => &[
        VerbDictionaryEntry { lemma: "krasti", addition: "(krade)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krasti sę" => &[
        VerbDictionaryEntry { lemma: "krasti sę", addition: "(krade)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kresati" => &[
        VerbDictionaryEntry { lemma: "kresati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krijumčariti" => &[
        VerbDictionaryEntry { lemma: "krijumčariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kriknųti" => &[
        VerbDictionaryEntry { lemma: "kriknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kristalizovati" => &[
        VerbDictionaryEntry { lemma: "kristalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kritikovati" => &[
        VerbDictionaryEntry { lemma: "kritikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krivditi" => &[
        VerbDictionaryEntry { lemma: "krivditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kriviti" => &[
        VerbDictionaryEntry { lemma: "kriviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kričati" => &[
        VerbDictionaryEntry { lemma: "kričati", addition: "(kriči)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krjakati" => &[
        VerbDictionaryEntry { lemma: "krjakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "krmiti" => &[
        VerbDictionaryEntry { lemma: "krmiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krojiti" => &[
        VerbDictionaryEntry { lemma: "krojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kropiti" => &[
        VerbDictionaryEntry { lemma: "kropiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "krotiti" => &[
        VerbDictionaryEntry { lemma: "krotiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krvaviti" => &[
        VerbDictionaryEntry { lemma: "krvaviti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kryti" => &[
        VerbDictionaryEntry { lemma: "kryti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kryti sę" => &[
        VerbDictionaryEntry { lemma: "kryti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "krčiti" => &[
        VerbDictionaryEntry { lemma: "krčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krėstiti" => &[
        VerbDictionaryEntry { lemma: "krėstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krěpiti" => &[
        VerbDictionaryEntry { lemma: "krěpiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krěpěti" => &[
        VerbDictionaryEntry { lemma: "krěpěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "krųtiti" => &[
        VerbDictionaryEntry { lemma: "krųtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krųtiti sę" => &[
        VerbDictionaryEntry { lemma: "krųtiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "krųžiti" => &[
        VerbDictionaryEntry { lemma: "krųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krųžiti sę" => &[
        VerbDictionaryEntry { lemma: "krųžiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "krȯšiti" => &[
        VerbDictionaryEntry { lemma: "krȯšiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "krȯšiti sę" => &[
        VerbDictionaryEntry { lemma: "krȯšiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kuhati" => &[
        VerbDictionaryEntry { lemma: "kuhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kukati" => &[
        VerbDictionaryEntry { lemma: "kukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kuljgati" => &[
        VerbDictionaryEntry { lemma: "kuljgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kulminovati" => &[
        VerbDictionaryEntry { lemma: "kulminovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kupiti" => &[
        VerbDictionaryEntry { lemma: "kupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kupovati" => &[
        VerbDictionaryEntry { lemma: "kupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kuriti" => &[
        VerbDictionaryEntry { lemma: "kuriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kustomizovati" => &[
        VerbDictionaryEntry { lemma: "kustomizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kvakati" => &[
        VerbDictionaryEntry { lemma: "kvakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kvalifikovati" => &[
        VerbDictionaryEntry { lemma: "kvalifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kvasiti" => &[
        VerbDictionaryEntry { lemma: "kvasiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kvičati" => &[
        VerbDictionaryEntry { lemma: "kvičati", addition: "(kviče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kvokati" => &[
        VerbDictionaryEntry { lemma: "kvokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kydati" => &[
        VerbDictionaryEntry { lemma: "kydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kydnųti" => &[
        VerbDictionaryEntry { lemma: "kydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kyhati" => &[
        VerbDictionaryEntry { lemma: "kyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kyhnųti" => &[
        VerbDictionaryEntry { lemma: "kyhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "kymati" => &[
        VerbDictionaryEntry { lemma: "kymati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kymnųti" => &[
        VerbDictionaryEntry { lemma: "kymnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "kypěti" => &[
        VerbDictionaryEntry { lemma: "kypěti", addition: "(kipi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kysnųti" => &[
        VerbDictionaryEntry { lemma: "kysnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kyvati" => &[
        VerbDictionaryEntry { lemma: "kyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "kyvnųti" => &[
        VerbDictionaryEntry { lemma: "kyvnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "kųpati" => &[
        VerbDictionaryEntry { lemma: "kųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kųpati sę" => &[
        VerbDictionaryEntry { lemma: "kųpati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "kųsati" => &[
        VerbDictionaryEntry { lemma: "kųsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "kųsnųti" => &[
        VerbDictionaryEntry { lemma: "kųsnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "kȯlzati sę" => &[
        VerbDictionaryEntry { lemma: "kȯlzati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "lajati" => &[
        VerbDictionaryEntry { lemma: "lajati", addition: "(laje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lakovati" => &[
        VerbDictionaryEntry { lemma: "lakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lamati" => &[
        VerbDictionaryEntry { lemma: "lamati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lapati" => &[
        VerbDictionaryEntry { lemma: "lapati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "laskati" => &[
        VerbDictionaryEntry { lemma: "laskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "latati" => &[
        VerbDictionaryEntry { lemma: "latati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "laziti" => &[
        VerbDictionaryEntry { lemma: "laziti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "leděniti" => &[
        VerbDictionaryEntry { lemma: "leděniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "leděněti" => &[
        VerbDictionaryEntry { lemma: "leděněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "legalizovati" => &[
        VerbDictionaryEntry { lemma: "legalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "legti" => &[
        VerbDictionaryEntry { lemma: "legti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "lepetati" => &[
        VerbDictionaryEntry { lemma: "lepetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "letěti" => &[
        VerbDictionaryEntry { lemma: "letěti", addition: "(leti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ležati" => &[
        VerbDictionaryEntry { lemma: "ležati", addition: "(leži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lgati" => &[
        VerbDictionaryEntry { lemma: "lgati", addition: "(lže)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "liberalizovati" => &[
        VerbDictionaryEntry { lemma: "liberalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "liceměriti" => &[
        VerbDictionaryEntry { lemma: "liceměriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "likvidovati" => &[
        VerbDictionaryEntry { lemma: "likvidovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "limitovati" => &[
        VerbDictionaryEntry { lemma: "limitovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "linjati" => &[
        VerbDictionaryEntry { lemma: "linjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "linčevati" => &[
        VerbDictionaryEntry { lemma: "linčevati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "liti" => &[
        VerbDictionaryEntry { lemma: "liti", addition: "(lije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lizati" => &[
        VerbDictionaryEntry { lemma: "lizati", addition: "(liže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "liznųti" => &[
        VerbDictionaryEntry { lemma: "liznųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "lišati" => &[
        VerbDictionaryEntry { lemma: "lišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lišiti" => &[
        VerbDictionaryEntry { lemma: "lišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ljstiti" => &[
        VerbDictionaryEntry { lemma: "ljstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ljubiti" => &[
        VerbDictionaryEntry { lemma: "ljubiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ljuljati" => &[
        VerbDictionaryEntry { lemma: "ljuljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lobovati" => &[
        VerbDictionaryEntry { lemma: "lobovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "logovati sę" => &[
        VerbDictionaryEntry { lemma: "logovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "lojiti" => &[
        VerbDictionaryEntry { lemma: "lojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lokati" => &[
        VerbDictionaryEntry { lemma: "lokati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "loknųti" => &[
        VerbDictionaryEntry { lemma: "loknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "lomiti" => &[
        VerbDictionaryEntry { lemma: "lomiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "loskotati" => &[
        VerbDictionaryEntry { lemma: "loskotati", addition: "(loskoče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "loviti" => &[
        VerbDictionaryEntry { lemma: "loviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lupiti" => &[
        VerbDictionaryEntry { lemma: "lupiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "luskati" => &[
        VerbDictionaryEntry { lemma: "luskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lučiti" => &[
        VerbDictionaryEntry { lemma: "lučiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "luščiti" => &[
        VerbDictionaryEntry { lemma: "luščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lysěti" => &[
        VerbDictionaryEntry { lemma: "lysěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lėskati" => &[
        VerbDictionaryEntry { lemma: "lėskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lėsknųti" => &[
        VerbDictionaryEntry { lemma: "lėsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "lęgati" => &[
        VerbDictionaryEntry { lemma: "lęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lęgti" => &[
        VerbDictionaryEntry { lemma: "lęgti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lęgti sę" => &[
        VerbDictionaryEntry { lemma: "lęgti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "lěniti sę" => &[
        VerbDictionaryEntry { lemma: "lěniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "lěpiti" => &[
        VerbDictionaryEntry { lemma: "lěpiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lěpiti sę" => &[
        VerbDictionaryEntry { lemma: "lěpiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "lětati" => &[
        VerbDictionaryEntry { lemma: "lětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lětovati" => &[
        VerbDictionaryEntry { lemma: "lětovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lězti" => &[
        VerbDictionaryEntry { lemma: "lězti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "lěčiti" => &[
        VerbDictionaryEntry { lemma: "lěčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "lěčiti sę" => &[
        VerbDictionaryEntry { lemma: "lěčiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mahati" => &[
        VerbDictionaryEntry { lemma: "mahati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mahnųti" => &[
        VerbDictionaryEntry { lemma: "mahnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "maljevati" => &[
        VerbDictionaryEntry { lemma: "maljevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "maltretovati" => &[
        VerbDictionaryEntry { lemma: "maltretovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "malěti" => &[
        VerbDictionaryEntry { lemma: "malěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mamiti" => &[
        VerbDictionaryEntry { lemma: "mamiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "manevrovati" => &[
        VerbDictionaryEntry { lemma: "manevrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "manipulovati" => &[
        VerbDictionaryEntry { lemma: "manipulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "marginalizovati" => &[
        VerbDictionaryEntry { lemma: "marginalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "marinovati" => &[
        VerbDictionaryEntry { lemma: "marinovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "marširovati" => &[
        VerbDictionaryEntry { lemma: "marširovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "masakrovati" => &[
        VerbDictionaryEntry { lemma: "masakrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "masirovati" => &[
        VerbDictionaryEntry { lemma: "masirovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "maskovati" => &[
        VerbDictionaryEntry { lemma: "maskovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "mastiti" => &[
        VerbDictionaryEntry { lemma: "mastiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "masturbovati" => &[
        VerbDictionaryEntry { lemma: "masturbovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mazati" => &[
        VerbDictionaryEntry { lemma: "mazati", addition: "(maže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "meblovati" => &[
        VerbDictionaryEntry { lemma: "meblovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "meditovati" => &[
        VerbDictionaryEntry { lemma: "meditovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mekati" => &[
        VerbDictionaryEntry { lemma: "mekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mesti" => &[
        VerbDictionaryEntry { lemma: "mesti", addition: "(mete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "metati" => &[
        VerbDictionaryEntry { lemma: "metati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "metnųti" => &[
        VerbDictionaryEntry { lemma: "metnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "mglati" => &[
        VerbDictionaryEntry { lemma: "mglati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "migati" => &[
        VerbDictionaryEntry { lemma: "migati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mignųti" => &[
        VerbDictionaryEntry { lemma: "mignųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "migrovati" => &[
        VerbDictionaryEntry { lemma: "migrovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "milovati" => &[
        VerbDictionaryEntry { lemma: "milovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "minimalizovati" => &[
        VerbDictionaryEntry { lemma: "minimalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "minovati" => &[
        VerbDictionaryEntry { lemma: "minovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "minųti" => &[
        VerbDictionaryEntry { lemma: "minųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "miriti" => &[
        VerbDictionaryEntry { lemma: "miriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mjaukati" => &[
        VerbDictionaryEntry { lemma: "mjaukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mjauknųti" => &[
        VerbDictionaryEntry { lemma: "mjauknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "mljaskati" => &[
        VerbDictionaryEntry { lemma: "mljaskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mlåděti" => &[
        VerbDictionaryEntry { lemma: "mlåděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mlåtiti" => &[
        VerbDictionaryEntry { lemma: "mlåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mlěti" => &[
        VerbDictionaryEntry { lemma: "mlěti", addition: "(melje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "množiti" => &[
        VerbDictionaryEntry { lemma: "množiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mněti" => &[
        VerbDictionaryEntry { lemma: "mněti", addition: "(mni)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mněvati" => &[
        VerbDictionaryEntry { lemma: "mněvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mobilizovati" => &[
        VerbDictionaryEntry { lemma: "mobilizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "modelovati" => &[
        VerbDictionaryEntry { lemma: "modelovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "modernizovati" => &[
        VerbDictionaryEntry { lemma: "modernizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moderovati" => &[
        VerbDictionaryEntry { lemma: "moderovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "modifikovati" => &[
        VerbDictionaryEntry { lemma: "modifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "modriti" => &[
        VerbDictionaryEntry { lemma: "modriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "modrěti" => &[
        VerbDictionaryEntry { lemma: "modrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mogti" => &[
        VerbDictionaryEntry { lemma: "mogti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moknųti" => &[
        VerbDictionaryEntry { lemma: "moknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mokriti" => &[
        VerbDictionaryEntry { lemma: "mokriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mokrěti" => &[
        VerbDictionaryEntry { lemma: "mokrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "molestovati" => &[
        VerbDictionaryEntry { lemma: "molestovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moliti sę" => &[
        VerbDictionaryEntry { lemma: "moliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "montovati" => &[
        VerbDictionaryEntry { lemma: "montovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moralizovati" => &[
        VerbDictionaryEntry { lemma: "moralizovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "morati" => &[
        VerbDictionaryEntry { lemma: "morati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moriti" => &[
        VerbDictionaryEntry { lemma: "moriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moriti sę" => &[
        VerbDictionaryEntry { lemma: "moriti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "motati" => &[
        VerbDictionaryEntry { lemma: "motati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "motivovati" => &[
        VerbDictionaryEntry { lemma: "motivovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "moćněti" => &[
        VerbDictionaryEntry { lemma: "moćněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "močiti" => &[
        VerbDictionaryEntry { lemma: "močiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "močiti sę" => &[
        VerbDictionaryEntry { lemma: "močiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mrdati" => &[
        VerbDictionaryEntry { lemma: "mrdati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mrdnųti" => &[
        VerbDictionaryEntry { lemma: "mrdnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "mrmjati" => &[
        VerbDictionaryEntry { lemma: "mrmjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mrznųti" => &[
        VerbDictionaryEntry { lemma: "mrznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mråziti" => &[
        VerbDictionaryEntry { lemma: "mråziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mråzosušati" => &[
        VerbDictionaryEntry { lemma: "mråzosušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mråzosušiti" => &[
        VerbDictionaryEntry { lemma: "mråzosušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "mråčiti sę" => &[
        VerbDictionaryEntry { lemma: "mråčiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mrčati" => &[
        VerbDictionaryEntry { lemma: "mrčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mrščiti" => &[
        VerbDictionaryEntry { lemma: "mrščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mrščiti sę" => &[
        VerbDictionaryEntry { lemma: "mrščiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mstiti" => &[
        VerbDictionaryEntry { lemma: "mstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mukati" => &[
        VerbDictionaryEntry { lemma: "mukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "musěti" => &[
        VerbDictionaryEntry { lemma: "musěti", addition: "(musi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mučati" => &[
        VerbDictionaryEntry { lemma: "mučati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "myliti" => &[
        VerbDictionaryEntry { lemma: "myliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "myliti sę" => &[
        VerbDictionaryEntry { lemma: "myliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mysliti" => &[
        VerbDictionaryEntry { lemma: "mysliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "myti" => &[
        VerbDictionaryEntry { lemma: "myti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "myškovati" => &[
        VerbDictionaryEntry { lemma: "myškovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mėdliti" => &[
        VerbDictionaryEntry { lemma: "mėdliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mėčtati" => &[
        VerbDictionaryEntry { lemma: "mėčtati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mękčeti" => &[
        VerbDictionaryEntry { lemma: "mękčeti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mękčiti" => &[
        VerbDictionaryEntry { lemma: "mękčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "męti" => &[
        VerbDictionaryEntry { lemma: "męti", addition: "(mne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "męčkati" => &[
        VerbDictionaryEntry { lemma: "męčkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "měnjati" => &[
        VerbDictionaryEntry { lemma: "měnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "měnjati sę" => &[
        VerbDictionaryEntry { lemma: "měnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "měriti" => &[
        VerbDictionaryEntry { lemma: "měriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "měriti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "měsiti" => &[
        VerbDictionaryEntry { lemma: "měsiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mětati" => &[
        VerbDictionaryEntry { lemma: "mětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "měšati" => &[
        VerbDictionaryEntry { lemma: "měšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mŕknųti" => &[
        VerbDictionaryEntry { lemma: "mŕknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mŕtvěti" => &[
        VerbDictionaryEntry { lemma: "mŕtvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mųdrěti" => &[
        VerbDictionaryEntry { lemma: "mųdrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mųtiti" => &[
        VerbDictionaryEntry { lemma: "mųtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mųtněti" => &[
        VerbDictionaryEntry { lemma: "mųtněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mųčiti" => &[
        VerbDictionaryEntry { lemma: "mųčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mųžiti sę" => &[
        VerbDictionaryEntry { lemma: "mųžiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "mȯlknųti" => &[
        VerbDictionaryEntry { lemma: "mȯlknųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "mȯlviti" => &[
        VerbDictionaryEntry { lemma: "mȯlviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "mȯlčati" => &[
        VerbDictionaryEntry { lemma: "mȯlčati", addition: "(mȯlči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nabajati" => &[
        VerbDictionaryEntry { lemma: "nabajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nabirati" => &[
        VerbDictionaryEntry { lemma: "nabirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nabiti" => &[
        VerbDictionaryEntry { lemma: "nabiti", addition: "(nabije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nabivati" => &[
        VerbDictionaryEntry { lemma: "nabivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nabrati" => &[
        VerbDictionaryEntry { lemma: "nabrati", addition: "(nabere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nabreknųti" => &[
        VerbDictionaryEntry { lemma: "nabreknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nabuhati" => &[
        VerbDictionaryEntry { lemma: "nabuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nabuhnųti" => &[
        VerbDictionaryEntry { lemma: "nabuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nabyti" => &[
        VerbDictionaryEntry { lemma: "nabyti", addition: "(nabųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nabyvati" => &[
        VerbDictionaryEntry { lemma: "nabyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nabzděti" => &[
        VerbDictionaryEntry { lemma: "nabzděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "naběgati sę" => &[
        VerbDictionaryEntry { lemma: "naběgati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nacionalizovati" => &[
        VerbDictionaryEntry { lemma: "nacionalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadati" => &[
        VerbDictionaryEntry { lemma: "nadati", addition: "(nada)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nadavati" => &[
        VerbDictionaryEntry { lemma: "nadavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadględati" => &[
        VerbDictionaryEntry { lemma: "nadględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadględěti" => &[
        VerbDictionaryEntry { lemma: "nadględěti", addition: "(nadględi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nadigrati" => &[
        VerbDictionaryEntry { lemma: "nadigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nadigryvati" => &[
        VerbDictionaryEntry { lemma: "nadigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadužiti" => &[
        VerbDictionaryEntry { lemma: "nadužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naduživati" => &[
        VerbDictionaryEntry { lemma: "naduživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadyhati" => &[
        VerbDictionaryEntry { lemma: "nadyhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadzirati" => &[
        VerbDictionaryEntry { lemma: "nadzirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadějati sę" => &[
        VerbDictionaryEntry { lemma: "nadějati sę", addition: "(naděje)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "nadělati" => &[
        VerbDictionaryEntry { lemma: "nadělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naděti" => &[
        VerbDictionaryEntry { lemma: "naděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naděvati" => &[
        VerbDictionaryEntry { lemma: "naděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadųti" => &[
        VerbDictionaryEntry { lemma: "nadųti", addition: "(nadme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nadųvati" => &[
        VerbDictionaryEntry { lemma: "nadųvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nadȯhnųti" => &[
        VerbDictionaryEntry { lemma: "nadȯhnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naganjati" => &[
        VerbDictionaryEntry { lemma: "naganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nagnati" => &[
        VerbDictionaryEntry { lemma: "nagnati", addition: "(nagone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagnojiti" => &[
        VerbDictionaryEntry { lemma: "nagnojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagnųti" => &[
        VerbDictionaryEntry { lemma: "nagnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagovoriti" => &[
        VerbDictionaryEntry { lemma: "nagovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagrađati" => &[
        VerbDictionaryEntry { lemma: "nagrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nagromaditi" => &[
        VerbDictionaryEntry { lemma: "nagromaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagromađati" => &[
        VerbDictionaryEntry { lemma: "nagromađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nagråditi" => &[
        VerbDictionaryEntry { lemma: "nagråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagrěti" => &[
        VerbDictionaryEntry { lemma: "nagrěti", addition: "(nagrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nagrěvati" => &[
        VerbDictionaryEntry { lemma: "nagrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nagybati" => &[
        VerbDictionaryEntry { lemma: "nagybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nahmuriti" => &[
        VerbDictionaryEntry { lemma: "nahmuriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nahoditi" => &[
        VerbDictionaryEntry { lemma: "nahoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nahoditi sę" => &[
        VerbDictionaryEntry { lemma: "nahoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "nahvatati" => &[
        VerbDictionaryEntry { lemma: "nahvatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naigrati" => &[
        VerbDictionaryEntry { lemma: "naigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naigryvati" => &[
        VerbDictionaryEntry { lemma: "naigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "najdti" => &[
        VerbDictionaryEntry { lemma: "najdti", addition: "(najde; našėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "najedati sę" => &[
        VerbDictionaryEntry { lemma: "najedati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "najesti sę" => &[
        VerbDictionaryEntry { lemma: "najesti sę", addition: "(naje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "naježiti" => &[
        VerbDictionaryEntry { lemma: "naježiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naježiti sę" => &[
        VerbDictionaryEntry { lemma: "naježiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "najmati" => &[
        VerbDictionaryEntry { lemma: "najmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "najęti" => &[
        VerbDictionaryEntry { lemma: "najęti", addition: "(najme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakalati" => &[
        VerbDictionaryEntry { lemma: "nakalati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nakapati" => &[
        VerbDictionaryEntry { lemma: "nakapati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nakazati" => &[
        VerbDictionaryEntry { lemma: "nakazati", addition: "(nakaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakazyvati" => &[
        VerbDictionaryEntry { lemma: "nakazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nakladati" => &[
        VerbDictionaryEntry { lemma: "nakladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naklanjati" => &[
        VerbDictionaryEntry { lemma: "naklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nakloniti" => &[
        VerbDictionaryEntry { lemma: "nakloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakopati" => &[
        VerbDictionaryEntry { lemma: "nakopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakopyvati" => &[
        VerbDictionaryEntry { lemma: "nakopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nakrmiti" => &[
        VerbDictionaryEntry { lemma: "nakrmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakryti" => &[
        VerbDictionaryEntry { lemma: "nakryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakryvati" => &[
        VerbDictionaryEntry { lemma: "nakryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nakydati" => &[
        VerbDictionaryEntry { lemma: "nakydati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nakydyvati" => &[
        VerbDictionaryEntry { lemma: "nakydyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nalagati" => &[
        VerbDictionaryEntry { lemma: "nalagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nalegati" => &[
        VerbDictionaryEntry { lemma: "nalegati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nalegti" => &[
        VerbDictionaryEntry { lemma: "nalegti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "naležati" => &[
        VerbDictionaryEntry { lemma: "naležati", addition: "(naleži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "naliti" => &[
        VerbDictionaryEntry { lemma: "naliti", addition: "(nalije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nalivati" => &[
        VerbDictionaryEntry { lemma: "nalivati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naložiti" => &[
        VerbDictionaryEntry { lemma: "naložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namastiti" => &[
        VerbDictionaryEntry { lemma: "namastiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namazati" => &[
        VerbDictionaryEntry { lemma: "namazati", addition: "(namaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namazyvati" => &[
        VerbDictionaryEntry { lemma: "namazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "namašćati" => &[
        VerbDictionaryEntry { lemma: "namašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "namoknųti" => &[
        VerbDictionaryEntry { lemma: "namoknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "namokriti" => &[
        VerbDictionaryEntry { lemma: "namokriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namontovati" => &[
        VerbDictionaryEntry { lemma: "namontovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namotati" => &[
        VerbDictionaryEntry { lemma: "namotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namotyvati" => &[
        VerbDictionaryEntry { lemma: "namotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "namočiti" => &[
        VerbDictionaryEntry { lemma: "namočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namyliti" => &[
        VerbDictionaryEntry { lemma: "namyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "namyljati" => &[
        VerbDictionaryEntry { lemma: "namyljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naměriti" => &[
        VerbDictionaryEntry { lemma: "naměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naměrjati" => &[
        VerbDictionaryEntry { lemma: "naměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nanesti" => &[
        VerbDictionaryEntry { lemma: "nanesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nanizati" => &[
        VerbDictionaryEntry { lemma: "nanizati", addition: "(naniže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nanositi" => &[
        VerbDictionaryEntry { lemma: "nanositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naostriti" => &[
        VerbDictionaryEntry { lemma: "naostriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napadati" => &[
        VerbDictionaryEntry { lemma: "napadati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napajati" => &[
        VerbDictionaryEntry { lemma: "napajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naparfumovati" => &[
        VerbDictionaryEntry { lemma: "naparfumovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naparfumovati sę" => &[
        VerbDictionaryEntry { lemma: "naparfumovati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "napasti" => &[
        VerbDictionaryEntry { lemma: "napasti", addition: "(napade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "napasti", addition: "(napase)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napasti sę" => &[
        VerbDictionaryEntry { lemma: "napasti sę", addition: "(napase)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "napečatati" => &[
        VerbDictionaryEntry { lemma: "napečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naphati" => &[
        VerbDictionaryEntry { lemma: "naphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napihati" => &[
        VerbDictionaryEntry { lemma: "napihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napinati" => &[
        VerbDictionaryEntry { lemma: "napinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napirati" => &[
        VerbDictionaryEntry { lemma: "napirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "napisati" => &[
        VerbDictionaryEntry { lemma: "napisati", addition: "(napiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napiti sę" => &[
        VerbDictionaryEntry { lemma: "napiti sę", addition: "(napije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "napivati sę" => &[
        VerbDictionaryEntry { lemma: "napivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "napljuvati" => &[
        VerbDictionaryEntry { lemma: "napljuvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "napojiti" => &[
        VerbDictionaryEntry { lemma: "napojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napominati" => &[
        VerbDictionaryEntry { lemma: "napominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napomněti" => &[
        VerbDictionaryEntry { lemma: "napomněti", addition: "(napomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napraviti" => &[
        VerbDictionaryEntry { lemma: "napraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napraviti sę" => &[
        VerbDictionaryEntry { lemma: "napraviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "napravjati" => &[
        VerbDictionaryEntry { lemma: "napravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napravjati sę" => &[
        VerbDictionaryEntry { lemma: "napravjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "naprěti" => &[
        VerbDictionaryEntry { lemma: "naprěti", addition: "(napre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "naprųžiti" => &[
        VerbDictionaryEntry { lemma: "naprųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napustiti" => &[
        VerbDictionaryEntry { lemma: "napustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napušćati" => &[
        VerbDictionaryEntry { lemma: "napušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "napęti" => &[
        VerbDictionaryEntry { lemma: "napęti", addition: "(napne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napŕti" => &[
        VerbDictionaryEntry { lemma: "napŕti", addition: "(napre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "napȯlniti" => &[
        VerbDictionaryEntry { lemma: "napȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "napȯlnjati" => &[
        VerbDictionaryEntry { lemma: "napȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "narastati" => &[
        VerbDictionaryEntry { lemma: "narastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "naroditi" => &[
        VerbDictionaryEntry { lemma: "naroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "narušati" => &[
        VerbDictionaryEntry { lemma: "narušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "narušiti" => &[
        VerbDictionaryEntry { lemma: "narušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "narvati" => &[
        VerbDictionaryEntry { lemma: "narvati", addition: "(narve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "narysovati" => &[
        VerbDictionaryEntry { lemma: "narysovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naryvati" => &[
        VerbDictionaryEntry { lemma: "naryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naråsti" => &[
        VerbDictionaryEntry { lemma: "naråsti", addition: "(naråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "narěkati" => &[
        VerbDictionaryEntry { lemma: "narěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "narěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "narěkti" => &[
        VerbDictionaryEntry { lemma: "narěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "narězati" => &[
        VerbDictionaryEntry { lemma: "narězati", addition: "(narěže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "narųčati" => &[
        VerbDictionaryEntry { lemma: "narųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "narųčiti" => &[
        VerbDictionaryEntry { lemma: "narųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nasaditi" => &[
        VerbDictionaryEntry { lemma: "nasaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nasađati" => &[
        VerbDictionaryEntry { lemma: "nasađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nascati" => &[
        VerbDictionaryEntry { lemma: "nascati", addition: "(nasci)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "naseliti" => &[
        VerbDictionaryEntry { lemma: "naseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naseljati" => &[
        VerbDictionaryEntry { lemma: "naseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nasilovati" => &[
        VerbDictionaryEntry { lemma: "nasilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naslađati" => &[
        VerbDictionaryEntry { lemma: "naslađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naslađati sę" => &[
        VerbDictionaryEntry { lemma: "naslađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "naslušati sę" => &[
        VerbDictionaryEntry { lemma: "naslušati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "naslåditi" => &[
        VerbDictionaryEntry { lemma: "naslåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naslåditi sę" => &[
        VerbDictionaryEntry { lemma: "naslåditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "naslěditi" => &[
        VerbDictionaryEntry { lemma: "naslěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naslědovati" => &[
        VerbDictionaryEntry { lemma: "naslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nasmoliti" => &[
        VerbDictionaryEntry { lemma: "nasmoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naspati sę" => &[
        VerbDictionaryEntry { lemma: "naspati sę", addition: "(naspi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nastati" => &[
        VerbDictionaryEntry { lemma: "nastati", addition: "(nastane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nastavati" => &[
        VerbDictionaryEntry { lemma: "nastavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nastaviti" => &[
        VerbDictionaryEntry { lemma: "nastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nastavjati" => &[
        VerbDictionaryEntry { lemma: "nastavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nastignųti" => &[
        VerbDictionaryEntry { lemma: "nastignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nastojati" => &[
        VerbDictionaryEntry { lemma: "nastojati", addition: "(nastoji)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nastojivati" => &[
        VerbDictionaryEntry { lemma: "nastojivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nastradati sę" => &[
        VerbDictionaryEntry { lemma: "nastradati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nastrajati" => &[
        VerbDictionaryEntry { lemma: "nastrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nastrašiti" => &[
        VerbDictionaryEntry { lemma: "nastrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nastrašiti sę" => &[
        VerbDictionaryEntry { lemma: "nastrašiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nastrojiti" => &[
        VerbDictionaryEntry { lemma: "nastrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nastråžiti sę" => &[
        VerbDictionaryEntry { lemma: "nastråžiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nastųpati" => &[
        VerbDictionaryEntry { lemma: "nastųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nastųpiti" => &[
        VerbDictionaryEntry { lemma: "nastųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nasypati" => &[
        VerbDictionaryEntry { lemma: "nasypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nasytiti" => &[
        VerbDictionaryEntry { lemma: "nasytiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nasyćati" => &[
        VerbDictionaryEntry { lemma: "nasyćati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nasěděti sę" => &[
        VerbDictionaryEntry { lemma: "nasěděti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "nasěkati" => &[
        VerbDictionaryEntry { lemma: "nasěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nasěkti" => &[
        VerbDictionaryEntry { lemma: "nasěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natipkati" => &[
        VerbDictionaryEntry { lemma: "natipkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natirati" => &[
        VerbDictionaryEntry { lemma: "natirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "natiskati" => &[
        VerbDictionaryEntry { lemma: "natiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "natisknųti" => &[
        VerbDictionaryEntry { lemma: "natisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natočiti" => &[
        VerbDictionaryEntry { lemma: "natočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natruditi sę" => &[
        VerbDictionaryEntry { lemma: "natruditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "natrěti" => &[
        VerbDictionaryEntry { lemma: "natrěti", addition: "(natre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natvoriti" => &[
        VerbDictionaryEntry { lemma: "natvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natęgati" => &[
        VerbDictionaryEntry { lemma: "natęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "natęgnųti" => &[
        VerbDictionaryEntry { lemma: "natęgnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "natěšiti sę" => &[
        VerbDictionaryEntry { lemma: "natěšiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "natŕpěti sę" => &[
        VerbDictionaryEntry { lemma: "natŕpěti sę", addition: "(natŕpi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "natŕti" => &[
        VerbDictionaryEntry { lemma: "natŕti", addition: "(natre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "natȯlkati sę" => &[
        VerbDictionaryEntry { lemma: "natȯlkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "natȯlkti sę" => &[
        VerbDictionaryEntry { lemma: "natȯlkti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "naučiti" => &[
        VerbDictionaryEntry { lemma: "naučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naučiti sę" => &[
        VerbDictionaryEntry { lemma: "naučiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "navesti" => &[
        VerbDictionaryEntry { lemma: "navesti", addition: "(navede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "naviti" => &[
        VerbDictionaryEntry { lemma: "naviti", addition: "(navije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navivati" => &[
        VerbDictionaryEntry { lemma: "navivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navlåžiti" => &[
        VerbDictionaryEntry { lemma: "navlåžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navlěkati" => &[
        VerbDictionaryEntry { lemma: "navlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navlěkti" => &[
        VerbDictionaryEntry { lemma: "navlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navoditi" => &[
        VerbDictionaryEntry { lemma: "navoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navodniti" => &[
        VerbDictionaryEntry { lemma: "navodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navodnjati" => &[
        VerbDictionaryEntry { lemma: "navodnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navoščiti" => &[
        VerbDictionaryEntry { lemma: "navoščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navråžiti" => &[
        VerbDictionaryEntry { lemma: "navråžiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "navęzati" => &[
        VerbDictionaryEntry { lemma: "navęzati", addition: "(navęže)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "navęzati", addition: "(navęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navęzyvati" => &[
        VerbDictionaryEntry { lemma: "navęzyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "navęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navěditi" => &[
        VerbDictionaryEntry { lemma: "navěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navěsiti" => &[
        VerbDictionaryEntry { lemma: "navěsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "navěđati" => &[
        VerbDictionaryEntry { lemma: "navěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "navěšati" => &[
        VerbDictionaryEntry { lemma: "navěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nazdravjati" => &[
        VerbDictionaryEntry { lemma: "nazdravjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nazdråviti" => &[
        VerbDictionaryEntry { lemma: "nazdråviti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "naznačati" => &[
        VerbDictionaryEntry { lemma: "naznačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "naznačiti" => &[
        VerbDictionaryEntry { lemma: "naznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nazvati" => &[
        VerbDictionaryEntry { lemma: "nazvati", addition: "(nazȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nazyvati" => &[
        VerbDictionaryEntry { lemma: "nazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nazyvati sę" => &[
        VerbDictionaryEntry { lemma: "nazyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "načaditi" => &[
        VerbDictionaryEntry { lemma: "načaditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "načinati" => &[
        VerbDictionaryEntry { lemma: "načinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "načinati sę" => &[
        VerbDictionaryEntry { lemma: "načinati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "načitati sę" => &[
        VerbDictionaryEntry { lemma: "načitati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "načrpati" => &[
        VerbDictionaryEntry { lemma: "načrpati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "načrtati" => &[
        VerbDictionaryEntry { lemma: "načrtati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "načęti" => &[
        VerbDictionaryEntry { lemma: "načęti", addition: "(načne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "načęti sę" => &[
        VerbDictionaryEntry { lemma: "načęti sę", addition: "(načne)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "našiti" => &[
        VerbDictionaryEntry { lemma: "našiti", addition: "(našije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "našivati" => &[
        VerbDictionaryEntry { lemma: "našivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "našėptati" => &[
        VerbDictionaryEntry { lemma: "našėptati", addition: "(našėpće)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nedoględnųti" => &[
        VerbDictionaryEntry { lemma: "nedoględnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nedoråzuměti" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nedoråzuměvati" => &[
        VerbDictionaryEntry { lemma: "nedoråzuměvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nedostavati" => &[
        VerbDictionaryEntry { lemma: "nedostavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nehati" => &[
        VerbDictionaryEntry { lemma: "nehati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nenaviděti" => &[
        VerbDictionaryEntry { lemma: "nenaviděti", addition: "(nenavidi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nepokojiti" => &[
        VerbDictionaryEntry { lemma: "nepokojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nepokojiti sę" => &[
        VerbDictionaryEntry { lemma: "nepokojiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "nesti" => &[
        VerbDictionaryEntry { lemma: "nesti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nevtralizovati" => &[
        VerbDictionaryEntry { lemma: "nevtralizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nezadovaljati" => &[
        VerbDictionaryEntry { lemma: "nezadovaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nezadovoliti" => &[
        VerbDictionaryEntry { lemma: "nezadovoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "nizati" => &[
        VerbDictionaryEntry { lemma: "nizati", addition: "(niže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "niščiti" => &[
        VerbDictionaryEntry { lemma: "niščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "njuhati" => &[
        VerbDictionaryEntry { lemma: "njuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "njuhnųti" => &[
        VerbDictionaryEntry { lemma: "njuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "nominovati" => &[
        VerbDictionaryEntry { lemma: "nominovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nositi" => &[
        VerbDictionaryEntry { lemma: "nositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "noćevati" => &[
        VerbDictionaryEntry { lemma: "noćevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nrěstiti sę" => &[
        VerbDictionaryEntry { lemma: "nrěstiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "nuditi" => &[
        VerbDictionaryEntry { lemma: "nuditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "nuditi sę" => &[
        VerbDictionaryEntry { lemma: "nuditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "nyrjati" => &[
        VerbDictionaryEntry { lemma: "nyrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "nyrnųti" => &[
        VerbDictionaryEntry { lemma: "nyrnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "něměti" => &[
        VerbDictionaryEntry { lemma: "něměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "obagriti" => &[
        VerbDictionaryEntry { lemma: "obagriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obdariti" => &[
        VerbDictionaryEntry { lemma: "obdariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obdarjati" => &[
        VerbDictionaryEntry { lemma: "obdarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obdirati" => &[
        VerbDictionaryEntry { lemma: "obdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obdivjati" => &[
        VerbDictionaryEntry { lemma: "obdivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obdrěti" => &[
        VerbDictionaryEntry { lemma: "obdrěti", addition: "(obdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obdumati" => &[
        VerbDictionaryEntry { lemma: "obdumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obdŕti" => &[
        VerbDictionaryEntry { lemma: "obdŕti", addition: "(obdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obezglåviti" => &[
        VerbDictionaryEntry { lemma: "obezglåviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obezglåvjati" => &[
        VerbDictionaryEntry { lemma: "obezglåvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obezhrabrjati" => &[
        VerbDictionaryEntry { lemma: "obezhrabrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obezhråbriti" => &[
        VerbDictionaryEntry { lemma: "obezhråbriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obezpokojiti" => &[
        VerbDictionaryEntry { lemma: "obezpokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obezsiliti" => &[
        VerbDictionaryEntry { lemma: "obezsiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obezsiljati" => &[
        VerbDictionaryEntry { lemma: "obezsiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obezuměti" => &[
        VerbDictionaryEntry { lemma: "obezuměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "obezuměvati" => &[
        VerbDictionaryEntry { lemma: "obezuměvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "obezčėstiti" => &[
        VerbDictionaryEntry { lemma: "obezčėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obglodati" => &[
        VerbDictionaryEntry { lemma: "obglodati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obględati" => &[
        VerbDictionaryEntry { lemma: "obględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obględěti" => &[
        VerbDictionaryEntry { lemma: "obględěti", addition: "(obględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obgovarjati" => &[
        VerbDictionaryEntry { lemma: "obgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obgovoriti" => &[
        VerbDictionaryEntry { lemma: "obgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obgryzati" => &[
        VerbDictionaryEntry { lemma: "obgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obgryzti" => &[
        VerbDictionaryEntry { lemma: "obgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obhoditi" => &[
        VerbDictionaryEntry { lemma: "obhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obhoditi sę" => &[
        VerbDictionaryEntry { lemma: "obhoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obhvatiti" => &[
        VerbDictionaryEntry { lemma: "obhvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obhvaćati" => &[
        VerbDictionaryEntry { lemma: "obhvaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obiděti" => &[
        VerbDictionaryEntry { lemma: "obiděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obimati" => &[
        VerbDictionaryEntry { lemma: "obimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obiđati" => &[
        VerbDictionaryEntry { lemma: "obiđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "objasniti" => &[
        VerbDictionaryEntry { lemma: "objasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "objasnjati" => &[
        VerbDictionaryEntry { lemma: "objasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "objaviti" => &[
        VerbDictionaryEntry { lemma: "objaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "objaviti sę" => &[
        VerbDictionaryEntry { lemma: "objaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "objavjati" => &[
        VerbDictionaryEntry { lemma: "objavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "objavjati sę" => &[
        VerbDictionaryEntry { lemma: "objavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "objedati sę" => &[
        VerbDictionaryEntry { lemma: "objedati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "objediniti" => &[
        VerbDictionaryEntry { lemma: "objediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "objedinjati" => &[
        VerbDictionaryEntry { lemma: "objedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "objehati" => &[
        VerbDictionaryEntry { lemma: "objehati", addition: "(objede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "objesti sę" => &[
        VerbDictionaryEntry { lemma: "objesti sę", addition: "(obje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "objezditi" => &[
        VerbDictionaryEntry { lemma: "objezditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obježđati" => &[
        VerbDictionaryEntry { lemma: "obježđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "objęti" => &[
        VerbDictionaryEntry { lemma: "objęti", addition: "(obȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obkaljati" => &[
        VerbDictionaryEntry { lemma: "obkaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obkoliti" => &[
        VerbDictionaryEntry { lemma: "obkoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obkradati" => &[
        VerbDictionaryEntry { lemma: "obkradati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obkrasti" => &[
        VerbDictionaryEntry { lemma: "obkrasti", addition: "(obkrade)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblagati" => &[
        VerbDictionaryEntry { lemma: "oblagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblajati" => &[
        VerbDictionaryEntry { lemma: "oblajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblajivati" => &[
        VerbDictionaryEntry { lemma: "oblajivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblamyvati" => &[
        VerbDictionaryEntry { lemma: "oblamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblačati" => &[
        VerbDictionaryEntry { lemma: "oblačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obleděněti" => &[
        VerbDictionaryEntry { lemma: "obleděněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "obletěti" => &[
        VerbDictionaryEntry { lemma: "obletěti", addition: "(obleti)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblinjati" => &[
        VerbDictionaryEntry { lemma: "oblinjati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obliti" => &[
        VerbDictionaryEntry { lemma: "obliti", addition: "(oblije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblivati" => &[
        VerbDictionaryEntry { lemma: "oblivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblizati" => &[
        VerbDictionaryEntry { lemma: "oblizati", addition: "(obliže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblizyvati" => &[
        VerbDictionaryEntry { lemma: "oblizyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblomiti" => &[
        VerbDictionaryEntry { lemma: "oblomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obložiti" => &[
        VerbDictionaryEntry { lemma: "obložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblupiti" => &[
        VerbDictionaryEntry { lemma: "oblupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obluščiti" => &[
        VerbDictionaryEntry { lemma: "obluščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblysěti" => &[
        VerbDictionaryEntry { lemma: "oblysěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oblåčiti" => &[
        VerbDictionaryEntry { lemma: "oblåčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblėgčati" => &[
        VerbDictionaryEntry { lemma: "oblėgčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblėgčiti" => &[
        VerbDictionaryEntry { lemma: "oblėgčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblěkati" => &[
        VerbDictionaryEntry { lemma: "oblěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oblěkti" => &[
        VerbDictionaryEntry { lemma: "oblěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oblětati" => &[
        VerbDictionaryEntry { lemma: "oblětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obmanyvati" => &[
        VerbDictionaryEntry { lemma: "obmanyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obmanųti" => &[
        VerbDictionaryEntry { lemma: "obmanųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obmazati" => &[
        VerbDictionaryEntry { lemma: "obmazati", addition: "(obmaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obmazyvati" => &[
        VerbDictionaryEntry { lemma: "obmazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obmeđati" => &[
        VerbDictionaryEntry { lemma: "obmeđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obmeđiti" => &[
        VerbDictionaryEntry { lemma: "obmeđiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obmotati" => &[
        VerbDictionaryEntry { lemma: "obmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obmotyvati" => &[
        VerbDictionaryEntry { lemma: "obmotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obmysliti" => &[
        VerbDictionaryEntry { lemma: "obmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obmysljati" => &[
        VerbDictionaryEntry { lemma: "obmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obměniti" => &[
        VerbDictionaryEntry { lemma: "obměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obměnjati" => &[
        VerbDictionaryEntry { lemma: "obměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnarodovati" => &[
        VerbDictionaryEntry { lemma: "obnarodovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obnavjati" => &[
        VerbDictionaryEntry { lemma: "obnavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnažati" => &[
        VerbDictionaryEntry { lemma: "obnažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnažati sę" => &[
        VerbDictionaryEntry { lemma: "obnažati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obnažiti" => &[
        VerbDictionaryEntry { lemma: "obnažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obnažiti sę" => &[
        VerbDictionaryEntry { lemma: "obnažiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obnesti" => &[
        VerbDictionaryEntry { lemma: "obnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obniziti" => &[
        VerbDictionaryEntry { lemma: "obniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obnižati" => &[
        VerbDictionaryEntry { lemma: "obnižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnjuhati" => &[
        VerbDictionaryEntry { lemma: "obnjuhati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obnjuhyvati" => &[
        VerbDictionaryEntry { lemma: "obnjuhyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnositi" => &[
        VerbDictionaryEntry { lemma: "obnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obnoviti" => &[
        VerbDictionaryEntry { lemma: "obnoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obobćati" => &[
        VerbDictionaryEntry { lemma: "obobćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obobćiti" => &[
        VerbDictionaryEntry { lemma: "obobćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obogatiti" => &[
        VerbDictionaryEntry { lemma: "obogatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obogatěti" => &[
        VerbDictionaryEntry { lemma: "obogatěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "obogaćati" => &[
        VerbDictionaryEntry { lemma: "obogaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obožati" => &[
        VerbDictionaryEntry { lemma: "obožati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obradovati" => &[
        VerbDictionaryEntry { lemma: "obradovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obradovati sę" => &[
        VerbDictionaryEntry { lemma: "obradovati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obranjati" => &[
        VerbDictionaryEntry { lemma: "obranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obraćati" => &[
        VerbDictionaryEntry { lemma: "obraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obraćati sę" => &[
        VerbDictionaryEntry { lemma: "obraćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obriti" => &[
        VerbDictionaryEntry { lemma: "obriti", addition: "(obrije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obriti sę" => &[
        VerbDictionaryEntry { lemma: "obriti sę", addition: "(obrije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obrušati sę" => &[
        VerbDictionaryEntry { lemma: "obrušati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obrušiti sę" => &[
        VerbDictionaryEntry { lemma: "obrušiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obrvati" => &[
        VerbDictionaryEntry { lemma: "obrvati", addition: "(obrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obrysovati" => &[
        VerbDictionaryEntry { lemma: "obrysovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obryvati" => &[
        VerbDictionaryEntry { lemma: "obryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obryzgati" => &[
        VerbDictionaryEntry { lemma: "obryzgati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obråbotati" => &[
        VerbDictionaryEntry { lemma: "obråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obråbotyvati" => &[
        VerbDictionaryEntry { lemma: "obråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obråniti" => &[
        VerbDictionaryEntry { lemma: "obråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obråtiti" => &[
        VerbDictionaryEntry { lemma: "obråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obråtiti sę" => &[
        VerbDictionaryEntry { lemma: "obråtiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obrěmeniti" => &[
        VerbDictionaryEntry { lemma: "obrěmeniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obrěmenjati" => &[
        VerbDictionaryEntry { lemma: "obrěmenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obrězati" => &[
        VerbDictionaryEntry { lemma: "obrězati", addition: "(obrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsaditi" => &[
        VerbDictionaryEntry { lemma: "obsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsađati" => &[
        VerbDictionaryEntry { lemma: "obsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obscati" => &[
        VerbDictionaryEntry { lemma: "obscati", addition: "(obsci)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "observovati" => &[
        VerbDictionaryEntry { lemma: "observovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obslugovati" => &[
        VerbDictionaryEntry { lemma: "obslugovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obslužiti" => &[
        VerbDictionaryEntry { lemma: "obslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obslědovati" => &[
        VerbDictionaryEntry { lemma: "obslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsrati" => &[
        VerbDictionaryEntry { lemma: "obsrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obstajati" => &[
        VerbDictionaryEntry { lemma: "obstajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "obstanavjati" => &[
        VerbDictionaryEntry { lemma: "obstanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obstanavjati sę" => &[
        VerbDictionaryEntry { lemma: "obstanavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obstanoviti sę" => &[
        VerbDictionaryEntry { lemma: "obstanoviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obstųpati" => &[
        VerbDictionaryEntry { lemma: "obstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obstųpiti" => &[
        VerbDictionaryEntry { lemma: "obstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsvětliti" => &[
        VerbDictionaryEntry { lemma: "obsvětliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsvětljati" => &[
        VerbDictionaryEntry { lemma: "obsvětljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsyhati" => &[
        VerbDictionaryEntry { lemma: "obsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "obsypati" => &[
        VerbDictionaryEntry { lemma: "obsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsypyvati" => &[
        VerbDictionaryEntry { lemma: "obsypyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsęgati" => &[
        VerbDictionaryEntry { lemma: "obsęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsęgnųti" => &[
        VerbDictionaryEntry { lemma: "obsęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsějati" => &[
        VerbDictionaryEntry { lemma: "obsějati", addition: "(obsěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsějivati" => &[
        VerbDictionaryEntry { lemma: "obsějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsěkati" => &[
        VerbDictionaryEntry { lemma: "obsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obsěkti" => &[
        VerbDictionaryEntry { lemma: "obsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "obsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "obtestovati" => &[
        VerbDictionaryEntry { lemma: "obtestovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obtirati" => &[
        VerbDictionaryEntry { lemma: "obtirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obtirati sę" => &[
        VerbDictionaryEntry { lemma: "obtirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obtrěti" => &[
        VerbDictionaryEntry { lemma: "obtrěti", addition: "(obtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obtrěti sę" => &[
        VerbDictionaryEntry { lemma: "obtrěti sę", addition: "(obtre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obtęžati" => &[
        VerbDictionaryEntry { lemma: "obtęžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obtęžiti" => &[
        VerbDictionaryEntry { lemma: "obtęžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obtŕti" => &[
        VerbDictionaryEntry { lemma: "obtŕti", addition: "(obtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obtŕti sę" => &[
        VerbDictionaryEntry { lemma: "obtŕti sę", addition: "(obtre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obuti sę" => &[
        VerbDictionaryEntry { lemma: "obuti sę", addition: "(obuje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obuvati sę" => &[
        VerbDictionaryEntry { lemma: "obuvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obuzdati" => &[
        VerbDictionaryEntry { lemma: "obuzdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obuzdyvati" => &[
        VerbDictionaryEntry { lemma: "obuzdyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obučati" => &[
        VerbDictionaryEntry { lemma: "obučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obučiti" => &[
        VerbDictionaryEntry { lemma: "obučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvadnjati" => &[
        VerbDictionaryEntry { lemma: "obvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obvažati" => &[
        VerbDictionaryEntry { lemma: "obvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obvažiti" => &[
        VerbDictionaryEntry { lemma: "obvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvesti" => &[
        VerbDictionaryEntry { lemma: "obvesti", addition: "(obvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obviti" => &[
        VerbDictionaryEntry { lemma: "obviti", addition: "(obvije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvivati" => &[
        VerbDictionaryEntry { lemma: "obvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obvoditi" => &[
        VerbDictionaryEntry { lemma: "obvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obvodniti" => &[
        VerbDictionaryEntry { lemma: "obvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvęzati" => &[
        VerbDictionaryEntry { lemma: "obvęzati", addition: "(obvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvęzyvati" => &[
        VerbDictionaryEntry { lemma: "obvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obvěstiti" => &[
        VerbDictionaryEntry { lemma: "obvěstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obvěšćati" => &[
        VerbDictionaryEntry { lemma: "obvěšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obyvati" => &[
        VerbDictionaryEntry { lemma: "obyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obzirati sę" => &[
        VerbDictionaryEntry { lemma: "obzirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obćiti" => &[
        VerbDictionaryEntry { lemma: "obćiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "občudovati" => &[
        VerbDictionaryEntry { lemma: "občudovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obědati" => &[
        VerbDictionaryEntry { lemma: "obědati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oběgati" => &[
        VerbDictionaryEntry { lemma: "oběgati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oběliti" => &[
        VerbDictionaryEntry { lemma: "oběliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obělěti" => &[
        VerbDictionaryEntry { lemma: "obělěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oběćati" => &[
        VerbDictionaryEntry { lemma: "oběćati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oběćivati" => &[
        VerbDictionaryEntry { lemma: "oběćivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obŕnųti" => &[
        VerbDictionaryEntry { lemma: "obŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obŕnųti sę" => &[
        VerbDictionaryEntry { lemma: "obŕnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obžegti" => &[
        VerbDictionaryEntry { lemma: "obžegti", addition: "(obžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obžigati" => &[
        VerbDictionaryEntry { lemma: "obžigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "obžirati sę" => &[
        VerbDictionaryEntry { lemma: "obžirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "obžrti sę" => &[
        VerbDictionaryEntry { lemma: "obžrti sę", addition: "(obžre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obžrěti sę" => &[
        VerbDictionaryEntry { lemma: "obžrěti sę", addition: "(obžre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obȯjdti" => &[
        VerbDictionaryEntry { lemma: "obȯjdti", addition: "(obȯjde; obšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "obȯjdti sę" => &[
        VerbDictionaryEntry { lemma: "obȯjdti sę", addition: "(obȯjde; obšėl)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "obȯzrěti sę" => &[
        VerbDictionaryEntry { lemma: "obȯzrěti sę", addition: "(obȯzri)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "ocěniti" => &[
        VerbDictionaryEntry { lemma: "ocěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odbirati" => &[
        VerbDictionaryEntry { lemma: "odbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odbiti" => &[
        VerbDictionaryEntry { lemma: "odbiti", addition: "(odbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odbiti sę" => &[
        VerbDictionaryEntry { lemma: "odbiti sę", addition: "(odbije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odbivati" => &[
        VerbDictionaryEntry { lemma: "odbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odbivati sę" => &[
        VerbDictionaryEntry { lemma: "odbivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odbrati" => &[
        VerbDictionaryEntry { lemma: "odbrati", addition: "(odbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odbyti sę" => &[
        VerbDictionaryEntry { lemma: "odbyti sę", addition: "(odbųde)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odbyvati sę" => &[
        VerbDictionaryEntry { lemma: "odbyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odběgati" => &[
        VerbDictionaryEntry { lemma: "odběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odběgti" => &[
        VerbDictionaryEntry { lemma: "odběgti", addition: "(odběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oddaliti" => &[
        VerbDictionaryEntry { lemma: "oddaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oddaliti sę" => &[
        VerbDictionaryEntry { lemma: "oddaliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "oddaljati" => &[
        VerbDictionaryEntry { lemma: "oddaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oddaljati sę" => &[
        VerbDictionaryEntry { lemma: "oddaljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "oddati" => &[
        VerbDictionaryEntry { lemma: "oddati", addition: "(odda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oddavati" => &[
        VerbDictionaryEntry { lemma: "oddavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oddirati" => &[
        VerbDictionaryEntry { lemma: "oddirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oddrěti" => &[
        VerbDictionaryEntry { lemma: "oddrěti", addition: "(oddre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oddyhati" => &[
        VerbDictionaryEntry { lemma: "oddyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odděliti" => &[
        VerbDictionaryEntry { lemma: "odděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odděljati" => &[
        VerbDictionaryEntry { lemma: "odděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oddŕti" => &[
        VerbDictionaryEntry { lemma: "oddŕti", addition: "(oddre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oddȯhnųti" => &[
        VerbDictionaryEntry { lemma: "oddȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odgadati" => &[
        VerbDictionaryEntry { lemma: "odgadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odgadyvati" => &[
        VerbDictionaryEntry { lemma: "odgadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odgnųti" => &[
        VerbDictionaryEntry { lemma: "odgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odgovarjati" => &[
        VerbDictionaryEntry { lemma: "odgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odgovoriti" => &[
        VerbDictionaryEntry { lemma: "odgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odgrebati" => &[
        VerbDictionaryEntry { lemma: "odgrebati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odgrebti" => &[
        VerbDictionaryEntry { lemma: "odgrebti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odgryzati" => &[
        VerbDictionaryEntry { lemma: "odgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odgryzti" => &[
        VerbDictionaryEntry { lemma: "odgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odgybati" => &[
        VerbDictionaryEntry { lemma: "odgybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odhoditi" => &[
        VerbDictionaryEntry { lemma: "odhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odidti" => &[
        VerbDictionaryEntry { lemma: "odidti", addition: "(odide; odšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odigrati sę" => &[
        VerbDictionaryEntry { lemma: "odigrati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odigryvati sę" => &[
        VerbDictionaryEntry { lemma: "odigryvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odimati" => &[
        VerbDictionaryEntry { lemma: "odimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odjaviti sę" => &[
        VerbDictionaryEntry { lemma: "odjaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odjavjati sę" => &[
        VerbDictionaryEntry { lemma: "odjavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odjehati" => &[
        VerbDictionaryEntry { lemma: "odjehati", addition: "(odjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odježđati" => &[
        VerbDictionaryEntry { lemma: "odježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odjęti" => &[
        VerbDictionaryEntry { lemma: "odjęti", addition: "(odȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkazati" => &[
        VerbDictionaryEntry { lemma: "odkazati", addition: "(odkaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkazati sę" => &[
        VerbDictionaryEntry { lemma: "odkazati sę", addition: "(odkaže)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odkazyvati" => &[
        VerbDictionaryEntry { lemma: "odkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odkazyvati sę" => &[
        VerbDictionaryEntry { lemma: "odkazyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odkladati" => &[
        VerbDictionaryEntry { lemma: "odkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odklanjati" => &[
        VerbDictionaryEntry { lemma: "odklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odklejati" => &[
        VerbDictionaryEntry { lemma: "odklejati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odklejati sę" => &[
        VerbDictionaryEntry { lemma: "odklejati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odklejiti" => &[
        VerbDictionaryEntry { lemma: "odklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odklejiti sę" => &[
        VerbDictionaryEntry { lemma: "odklejiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odključati" => &[
        VerbDictionaryEntry { lemma: "odključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odključati sę" => &[
        VerbDictionaryEntry { lemma: "odključati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odključiti" => &[
        VerbDictionaryEntry { lemma: "odključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odključiti sę" => &[
        VerbDictionaryEntry { lemma: "odključiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odkloniti" => &[
        VerbDictionaryEntry { lemma: "odkloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkryti" => &[
        VerbDictionaryEntry { lemma: "odkryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkryvati" => &[
        VerbDictionaryEntry { lemma: "odkryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odkupiti" => &[
        VerbDictionaryEntry { lemma: "odkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkydati" => &[
        VerbDictionaryEntry { lemma: "odkydati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odkydnųti" => &[
        VerbDictionaryEntry { lemma: "odkydnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkųsiti" => &[
        VerbDictionaryEntry { lemma: "odkųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odkųšati" => &[
        VerbDictionaryEntry { lemma: "odkųšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odlamyvati" => &[
        VerbDictionaryEntry { lemma: "odlamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odletěti" => &[
        VerbDictionaryEntry { lemma: "odletěti", addition: "(odleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odliti" => &[
        VerbDictionaryEntry { lemma: "odliti", addition: "(odlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odlivati" => &[
        VerbDictionaryEntry { lemma: "odlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odličati" => &[
        VerbDictionaryEntry { lemma: "odličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odličiti" => &[
        VerbDictionaryEntry { lemma: "odličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odlogovati sę" => &[
        VerbDictionaryEntry { lemma: "odlogovati sę", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odlomiti" => &[
        VerbDictionaryEntry { lemma: "odlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odložiti" => &[
        VerbDictionaryEntry { lemma: "odložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odlěpiti" => &[
        VerbDictionaryEntry { lemma: "odlěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odlěpiti sę" => &[
        VerbDictionaryEntry { lemma: "odlěpiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odlěpjati" => &[
        VerbDictionaryEntry { lemma: "odlěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odlěpjati sę" => &[
        VerbDictionaryEntry { lemma: "odlěpjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odlětati" => &[
        VerbDictionaryEntry { lemma: "odlětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odmesti" => &[
        VerbDictionaryEntry { lemma: "odmesti", addition: "(odmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odmetati" => &[
        VerbDictionaryEntry { lemma: "odmetati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odmetnųti" => &[
        VerbDictionaryEntry { lemma: "odmetnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odmstiti" => &[
        VerbDictionaryEntry { lemma: "odmstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odmětati" => &[
        VerbDictionaryEntry { lemma: "odmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odnesti" => &[
        VerbDictionaryEntry { lemma: "odnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odnositi" => &[
        VerbDictionaryEntry { lemma: "odnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odobriti" => &[
        VerbDictionaryEntry { lemma: "odobriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odobrjati" => &[
        VerbDictionaryEntry { lemma: "odobrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odolěti" => &[
        VerbDictionaryEntry { lemma: "odolěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odolěvati" => &[
        VerbDictionaryEntry { lemma: "odolěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odomašniti" => &[
        VerbDictionaryEntry { lemma: "odomašniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odomašnjati" => &[
        VerbDictionaryEntry { lemma: "odomašnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odosobniti" => &[
        VerbDictionaryEntry { lemma: "odosobniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odosobnjati" => &[
        VerbDictionaryEntry { lemma: "odosobnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpadati" => &[
        VerbDictionaryEntry { lemma: "odpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odpasti" => &[
        VerbDictionaryEntry { lemma: "odpasti", addition: "(odpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odpečatati" => &[
        VerbDictionaryEntry { lemma: "odpečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpečatati sę" => &[
        VerbDictionaryEntry { lemma: "odpečatati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odpečatyvati" => &[
        VerbDictionaryEntry { lemma: "odpečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpečatyvati sę" => &[
        VerbDictionaryEntry { lemma: "odpečatyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odphati" => &[
        VerbDictionaryEntry { lemma: "odphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpihati" => &[
        VerbDictionaryEntry { lemma: "odpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpiliti" => &[
        VerbDictionaryEntry { lemma: "odpiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpiljati" => &[
        VerbDictionaryEntry { lemma: "odpiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpinati" => &[
        VerbDictionaryEntry { lemma: "odpinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpirati sę" => &[
        VerbDictionaryEntry { lemma: "odpirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odpisati" => &[
        VerbDictionaryEntry { lemma: "odpisati", addition: "(odpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpisyvati" => &[
        VerbDictionaryEntry { lemma: "odpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odplatiti" => &[
        VerbDictionaryEntry { lemma: "odplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odplaćati" => &[
        VerbDictionaryEntry { lemma: "odplaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odplesti" => &[
        VerbDictionaryEntry { lemma: "odplesti", addition: "(odplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpletati" => &[
        VerbDictionaryEntry { lemma: "odpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpluti" => &[
        VerbDictionaryEntry { lemma: "odpluti", addition: "(odplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odplyvati" => &[
        VerbDictionaryEntry { lemma: "odplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odplyvti" => &[
        VerbDictionaryEntry { lemma: "odplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odpovědati" => &[
        VerbDictionaryEntry { lemma: "odpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "odpovědati", addition: "(+3)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(3) },
    ],
    "odpověděti" => &[
        VerbDictionaryEntry { lemma: "odpověděti", addition: "(odpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpočivati" => &[
        VerbDictionaryEntry { lemma: "odpočivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odpočęti" => &[
        VerbDictionaryEntry { lemma: "odpočęti", addition: "(odpočne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odpraviti" => &[
        VerbDictionaryEntry { lemma: "odpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpraviti sę" => &[
        VerbDictionaryEntry { lemma: "odpraviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odpravjati" => &[
        VerbDictionaryEntry { lemma: "odpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpravjati sę" => &[
        VerbDictionaryEntry { lemma: "odpravjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odprašati" => &[
        VerbDictionaryEntry { lemma: "odprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odprašati sę" => &[
        VerbDictionaryEntry { lemma: "odprašati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odprositi sę" => &[
        VerbDictionaryEntry { lemma: "odprositi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odpråšiti" => &[
        VerbDictionaryEntry { lemma: "odpråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odprěti sę" => &[
        VerbDictionaryEntry { lemma: "odprěti sę", addition: "(odpre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odpustiti" => &[
        VerbDictionaryEntry { lemma: "odpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpušćati" => &[
        VerbDictionaryEntry { lemma: "odpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odpęti" => &[
        VerbDictionaryEntry { lemma: "odpęti", addition: "(odpne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odpŕti sę" => &[
        VerbDictionaryEntry { lemma: "odpŕti sę", addition: "(odpre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odpųditi" => &[
        VerbDictionaryEntry { lemma: "odpųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odraditi" => &[
        VerbDictionaryEntry { lemma: "odraditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odraziti" => &[
        VerbDictionaryEntry { lemma: "odraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odraziti sę" => &[
        VerbDictionaryEntry { lemma: "odraziti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odračati" => &[
        VerbDictionaryEntry { lemma: "odračati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odrađati" => &[
        VerbDictionaryEntry { lemma: "odrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odražati" => &[
        VerbDictionaryEntry { lemma: "odražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odražati sę" => &[
        VerbDictionaryEntry { lemma: "odražati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odrestavrovati" => &[
        VerbDictionaryEntry { lemma: "odrestavrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odročiti" => &[
        VerbDictionaryEntry { lemma: "odročiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odrvati" => &[
        VerbDictionaryEntry { lemma: "odrvati", addition: "(odrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odryti" => &[
        VerbDictionaryEntry { lemma: "odryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odryvati" => &[
        VerbDictionaryEntry { lemma: "odryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odrěkati" => &[
        VerbDictionaryEntry { lemma: "odrěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odrěkati sę" => &[
        VerbDictionaryEntry { lemma: "odrěkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odrěkti" => &[
        VerbDictionaryEntry { lemma: "odrěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odrěkti sę" => &[
        VerbDictionaryEntry { lemma: "odrěkti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odrězati" => &[
        VerbDictionaryEntry { lemma: "odrězati", addition: "(odrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odrězyvati" => &[
        VerbDictionaryEntry { lemma: "odrězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odrųbati" => &[
        VerbDictionaryEntry { lemma: "odrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odrųbyvati" => &[
        VerbDictionaryEntry { lemma: "odrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odskakati" => &[
        VerbDictionaryEntry { lemma: "odskakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odskočiti" => &[
        VerbDictionaryEntry { lemma: "odskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odslanjati" => &[
        VerbDictionaryEntry { lemma: "odslanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odslati" => &[
        VerbDictionaryEntry { lemma: "odslati", addition: "(odšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odsloniti" => &[
        VerbDictionaryEntry { lemma: "odsloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odslužiti" => &[
        VerbDictionaryEntry { lemma: "odslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstati" => &[
        VerbDictionaryEntry { lemma: "odstati", addition: "(odstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odstavati" => &[
        VerbDictionaryEntry { lemma: "odstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odstaviti" => &[
        VerbDictionaryEntry { lemma: "odstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstavjati" => &[
        VerbDictionaryEntry { lemma: "odstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odstranjati" => &[
        VerbDictionaryEntry { lemma: "odstranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odstrašati" => &[
        VerbDictionaryEntry { lemma: "odstrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odstrašiti" => &[
        VerbDictionaryEntry { lemma: "odstrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstrigati" => &[
        VerbDictionaryEntry { lemma: "odstrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odstrigti" => &[
        VerbDictionaryEntry { lemma: "odstrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstråniti" => &[
        VerbDictionaryEntry { lemma: "odstråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstrěliti" => &[
        VerbDictionaryEntry { lemma: "odstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odstųpati" => &[
        VerbDictionaryEntry { lemma: "odstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "odstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odstųpiti" => &[
        VerbDictionaryEntry { lemma: "odstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "odstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odsunųti" => &[
        VerbDictionaryEntry { lemma: "odsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odsuvati" => &[
        VerbDictionaryEntry { lemma: "odsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odsyhati" => &[
        VerbDictionaryEntry { lemma: "odsyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odsylati" => &[
        VerbDictionaryEntry { lemma: "odsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odsěděti" => &[
        VerbDictionaryEntry { lemma: "odsěděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odsěkati" => &[
        VerbDictionaryEntry { lemma: "odsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odsěkti" => &[
        VerbDictionaryEntry { lemma: "odsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odsěđati" => &[
        VerbDictionaryEntry { lemma: "odsěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odsųtstvovati" => &[
        VerbDictionaryEntry { lemma: "odsųtstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odsȯhnųti" => &[
        VerbDictionaryEntry { lemma: "odsȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odtajati" => &[
        VerbDictionaryEntry { lemma: "odtajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odtekti" => &[
        VerbDictionaryEntry { lemma: "odtekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odtirati" => &[
        VerbDictionaryEntry { lemma: "odtirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odtiskati" => &[
        VerbDictionaryEntry { lemma: "odtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odtiskati sę" => &[
        VerbDictionaryEntry { lemma: "odtiskati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odtisknųti" => &[
        VerbDictionaryEntry { lemma: "odtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odtisknųti sę" => &[
        VerbDictionaryEntry { lemma: "odtisknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odtrěti" => &[
        VerbDictionaryEntry { lemma: "odtrěti", addition: "(odtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odtęgati" => &[
        VerbDictionaryEntry { lemma: "odtęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odtęgnųti" => &[
        VerbDictionaryEntry { lemma: "odtęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odtěkati" => &[
        VerbDictionaryEntry { lemma: "odtěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "odtŕti" => &[
        VerbDictionaryEntry { lemma: "odtŕti", addition: "(odtre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odurěti" => &[
        VerbDictionaryEntry { lemma: "odurěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "odučati" => &[
        VerbDictionaryEntry { lemma: "odučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odučiti" => &[
        VerbDictionaryEntry { lemma: "odučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvadnjati" => &[
        VerbDictionaryEntry { lemma: "odvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvesti" => &[
        VerbDictionaryEntry { lemma: "odvesti", addition: "(odvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvezti" => &[
        VerbDictionaryEntry { lemma: "odvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvinųti" => &[
        VerbDictionaryEntry { lemma: "odvinųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvivati" => &[
        VerbDictionaryEntry { lemma: "odvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvlåčivati" => &[
        VerbDictionaryEntry { lemma: "odvlåčivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvlěkati" => &[
        VerbDictionaryEntry { lemma: "odvlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvlěkti" => &[
        VerbDictionaryEntry { lemma: "odvlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvoditi" => &[
        VerbDictionaryEntry { lemma: "odvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvodniti" => &[
        VerbDictionaryEntry { lemma: "odvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvoziti" => &[
        VerbDictionaryEntry { lemma: "odvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvraćati" => &[
        VerbDictionaryEntry { lemma: "odvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvråtiti" => &[
        VerbDictionaryEntry { lemma: "odvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvykati" => &[
        VerbDictionaryEntry { lemma: "odvykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvykati sę" => &[
        VerbDictionaryEntry { lemma: "odvykati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odvyknųti" => &[
        VerbDictionaryEntry { lemma: "odvyknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvyknųti sę" => &[
        VerbDictionaryEntry { lemma: "odvyknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odvęzati" => &[
        VerbDictionaryEntry { lemma: "odvęzati", addition: "(odvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odvęzyvati" => &[
        VerbDictionaryEntry { lemma: "odvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvŕgati" => &[
        VerbDictionaryEntry { lemma: "odvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "odvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odzavisiti" => &[
        VerbDictionaryEntry { lemma: "odzavisiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odzvati" => &[
        VerbDictionaryEntry { lemma: "odzvati", addition: "(odzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odzvati sę" => &[
        VerbDictionaryEntry { lemma: "odzvati sę", addition: "(odzȯve)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odzyvati" => &[
        VerbDictionaryEntry { lemma: "odzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odzyvati sę" => &[
        VerbDictionaryEntry { lemma: "odzyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odćuđati" => &[
        VerbDictionaryEntry { lemma: "odćuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odćuđiti" => &[
        VerbDictionaryEntry { lemma: "odćuđiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odčajati" => &[
        VerbDictionaryEntry { lemma: "odčajati", addition: "(odčaje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odčajati sę" => &[
        VerbDictionaryEntry { lemma: "odčajati sę", addition: "(odčaje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "odčajivati" => &[
        VerbDictionaryEntry { lemma: "odčajivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odčajivati sę" => &[
        VerbDictionaryEntry { lemma: "odčajivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "odčiniti" => &[
        VerbDictionaryEntry { lemma: "odčiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odčinjati" => &[
        VerbDictionaryEntry { lemma: "odčinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odčitati" => &[
        VerbDictionaryEntry { lemma: "odčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odčityvati" => &[
        VerbDictionaryEntry { lemma: "odčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odčuvati" => &[
        VerbDictionaryEntry { lemma: "odčuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oděti" => &[
        VerbDictionaryEntry { lemma: "oděti", addition: "(oděne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oděvati" => &[
        VerbDictionaryEntry { lemma: "oděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odšlupati" => &[
        VerbDictionaryEntry { lemma: "odšlupati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odšlupyvati" => &[
        VerbDictionaryEntry { lemma: "odšlupyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "odščepiti" => &[
        VerbDictionaryEntry { lemma: "odščepiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "odščepjati" => &[
        VerbDictionaryEntry { lemma: "odščepjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogladiti" => &[
        VerbDictionaryEntry { lemma: "ogladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oglašati" => &[
        VerbDictionaryEntry { lemma: "oglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogluhnųti" => &[
        VerbDictionaryEntry { lemma: "ogluhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oglupěti" => &[
        VerbDictionaryEntry { lemma: "oglupěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oglušeti" => &[
        VerbDictionaryEntry { lemma: "oglušeti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oglušiti" => &[
        VerbDictionaryEntry { lemma: "oglušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oglåsiti" => &[
        VerbDictionaryEntry { lemma: "oglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oględati" => &[
        VerbDictionaryEntry { lemma: "oględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oględěti" => &[
        VerbDictionaryEntry { lemma: "oględěti", addition: "(oględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ognojiti" => &[
        VerbDictionaryEntry { lemma: "ognojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogoliti" => &[
        VerbDictionaryEntry { lemma: "ogoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogoliti sę" => &[
        VerbDictionaryEntry { lemma: "ogoliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "ogorčati" => &[
        VerbDictionaryEntry { lemma: "ogorčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogorčiti" => &[
        VerbDictionaryEntry { lemma: "ogorčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogovarjati" => &[
        VerbDictionaryEntry { lemma: "ogovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogovoriti" => &[
        VerbDictionaryEntry { lemma: "ogovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ograbiti" => &[
        VerbDictionaryEntry { lemma: "ograbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ograbjati" => &[
        VerbDictionaryEntry { lemma: "ograbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ograničati" => &[
        VerbDictionaryEntry { lemma: "ograničati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ograničiti" => &[
        VerbDictionaryEntry { lemma: "ograničiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ograđati" => &[
        VerbDictionaryEntry { lemma: "ograđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogražati" => &[
        VerbDictionaryEntry { lemma: "ogražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ogroziti" => &[
        VerbDictionaryEntry { lemma: "ogroziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogråditi" => &[
        VerbDictionaryEntry { lemma: "ogråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogrěti" => &[
        VerbDictionaryEntry { lemma: "ogrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ogrěvati" => &[
        VerbDictionaryEntry { lemma: "ogrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ohati" => &[
        VerbDictionaryEntry { lemma: "ohati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ohlađati" => &[
        VerbDictionaryEntry { lemma: "ohlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ohlåditi" => &[
        VerbDictionaryEntry { lemma: "ohlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ohlåděti" => &[
        VerbDictionaryEntry { lemma: "ohlåděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ohranjati" => &[
        VerbDictionaryEntry { lemma: "ohranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ohroměti" => &[
        VerbDictionaryEntry { lemma: "ohroměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ohråniti" => &[
        VerbDictionaryEntry { lemma: "ohråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okameniti" => &[
        VerbDictionaryEntry { lemma: "okameniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okamenjati" => &[
        VerbDictionaryEntry { lemma: "okamenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "okameněti" => &[
        VerbDictionaryEntry { lemma: "okameněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "okazati" => &[
        VerbDictionaryEntry { lemma: "okazati", addition: "(okaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okazati sę" => &[
        VerbDictionaryEntry { lemma: "okazati sę", addition: "(okaže)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "okazyvati" => &[
        VerbDictionaryEntry { lemma: "okazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "okazyvati sę" => &[
        VerbDictionaryEntry { lemma: "okazyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "oklevetati" => &[
        VerbDictionaryEntry { lemma: "oklevetati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okolorovati" => &[
        VerbDictionaryEntry { lemma: "okolorovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okotiti sę" => &[
        VerbDictionaryEntry { lemma: "okotiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "okovati" => &[
        VerbDictionaryEntry { lemma: "okovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okropiti" => &[
        VerbDictionaryEntry { lemma: "okropiti", addition: "(+5)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: Some(5) },
    ],
    "okropjati" => &[
        VerbDictionaryEntry { lemma: "okropjati", addition: "(+5)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(5) },
    ],
    "okrėstiti" => &[
        VerbDictionaryEntry { lemma: "okrėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okrěpnųti" => &[
        VerbDictionaryEntry { lemma: "okrěpnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "okrųgliti" => &[
        VerbDictionaryEntry { lemma: "okrųgliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okrųgljati" => &[
        VerbDictionaryEntry { lemma: "okrųgljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "okrųžati" => &[
        VerbDictionaryEntry { lemma: "okrųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "okrųžiti" => &[
        VerbDictionaryEntry { lemma: "okrųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "okupovati" => &[
        VerbDictionaryEntry { lemma: "okupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "omamiti" => &[
        VerbDictionaryEntry { lemma: "omamiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omastiti" => &[
        VerbDictionaryEntry { lemma: "omastiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omesti" => &[
        VerbDictionaryEntry { lemma: "omesti", addition: "(omete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omlađati" => &[
        VerbDictionaryEntry { lemma: "omlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "omlåditi" => &[
        VerbDictionaryEntry { lemma: "omlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omlåděti" => &[
        VerbDictionaryEntry { lemma: "omlåděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "omlěti" => &[
        VerbDictionaryEntry { lemma: "omlěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "omlěvati" => &[
        VerbDictionaryEntry { lemma: "omlěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "omyti" => &[
        VerbDictionaryEntry { lemma: "omyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omětati" => &[
        VerbDictionaryEntry { lemma: "omětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "omŕtvěti" => &[
        VerbDictionaryEntry { lemma: "omŕtvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "omŕziti" => &[
        VerbDictionaryEntry { lemma: "omŕziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omŕžati" => &[
        VerbDictionaryEntry { lemma: "omŕžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "omųžiti" => &[
        VerbDictionaryEntry { lemma: "omųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "omųžiti sę" => &[
        VerbDictionaryEntry { lemma: "omųžiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "onesměliti" => &[
        VerbDictionaryEntry { lemma: "onesměliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "onesměljati" => &[
        VerbDictionaryEntry { lemma: "onesměljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oněměti" => &[
        VerbDictionaryEntry { lemma: "oněměti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "opadati" => &[
        VerbDictionaryEntry { lemma: "opadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "opakovati" => &[
        VerbDictionaryEntry { lemma: "opakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opakovyvati" => &[
        VerbDictionaryEntry { lemma: "opakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opaliti" => &[
        VerbDictionaryEntry { lemma: "opaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opaljati" => &[
        VerbDictionaryEntry { lemma: "opaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opariti" => &[
        VerbDictionaryEntry { lemma: "opariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oparjati" => &[
        VerbDictionaryEntry { lemma: "oparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opasti" => &[
        VerbDictionaryEntry { lemma: "opasti", addition: "(opade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "opekati" => &[
        VerbDictionaryEntry { lemma: "opekati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "operiti sę" => &[
        VerbDictionaryEntry { lemma: "operiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "operjati sę" => &[
        VerbDictionaryEntry { lemma: "operjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "opirati sę" => &[
        VerbDictionaryEntry { lemma: "opirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "opisati" => &[
        VerbDictionaryEntry { lemma: "opisati", addition: "(opiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opisyvati" => &[
        VerbDictionaryEntry { lemma: "opisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opiti" => &[
        VerbDictionaryEntry { lemma: "opiti", addition: "(opije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opiti sę" => &[
        VerbDictionaryEntry { lemma: "opiti sę", addition: "(opije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "opivati" => &[
        VerbDictionaryEntry { lemma: "opivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opivati sę" => &[
        VerbDictionaryEntry { lemma: "opivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "oplakati" => &[
        VerbDictionaryEntry { lemma: "oplakati", addition: "(oplače)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oplakyvati" => &[
        VerbDictionaryEntry { lemma: "oplakyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opljunųti" => &[
        VerbDictionaryEntry { lemma: "opljunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opljuvati" => &[
        VerbDictionaryEntry { lemma: "opljuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oploditi" => &[
        VerbDictionaryEntry { lemma: "oploditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oplođati" => &[
        VerbDictionaryEntry { lemma: "oplođati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opluti" => &[
        VerbDictionaryEntry { lemma: "opluti", addition: "(oplove)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oplyvati" => &[
        VerbDictionaryEntry { lemma: "oplyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oplěti" => &[
        VerbDictionaryEntry { lemma: "oplěti", addition: "(oplěje/oplěve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oporędčati" => &[
        VerbDictionaryEntry { lemma: "oporędčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oporędčiti" => &[
        VerbDictionaryEntry { lemma: "oporędčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opozdniti" => &[
        VerbDictionaryEntry { lemma: "opozdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opozdniti sę" => &[
        VerbDictionaryEntry { lemma: "opozdniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "opozdnjati" => &[
        VerbDictionaryEntry { lemma: "opozdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opozdnjati sę" => &[
        VerbDictionaryEntry { lemma: "opozdnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "opoznati" => &[
        VerbDictionaryEntry { lemma: "opoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opoznavati" => &[
        VerbDictionaryEntry { lemma: "opoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opravdati" => &[
        VerbDictionaryEntry { lemma: "opravdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opravdyvati" => &[
        VerbDictionaryEntry { lemma: "opravdyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opraviti" => &[
        VerbDictionaryEntry { lemma: "opraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opravjati" => &[
        VerbDictionaryEntry { lemma: "opravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opravniti" => &[
        VerbDictionaryEntry { lemma: "opravniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opravnjati" => &[
        VerbDictionaryEntry { lemma: "opravnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oprovŕgati" => &[
        VerbDictionaryEntry { lemma: "oprovŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oprovŕgnųti" => &[
        VerbDictionaryEntry { lemma: "oprovŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opråzdniti" => &[
        VerbDictionaryEntry { lemma: "opråzdniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opråzdnjati" => &[
        VerbDictionaryEntry { lemma: "opråzdnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oprěděliti" => &[
        VerbDictionaryEntry { lemma: "oprěděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oprěděljati" => &[
        VerbDictionaryEntry { lemma: "oprěděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oprěti sę" => &[
        VerbDictionaryEntry { lemma: "oprěti sę", addition: "(opre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "optimizovati" => &[
        VerbDictionaryEntry { lemma: "optimizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opublikovati" => &[
        VerbDictionaryEntry { lemma: "opublikovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opuhati" => &[
        VerbDictionaryEntry { lemma: "opuhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "opuhnųti" => &[
        VerbDictionaryEntry { lemma: "opuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "opustašati" => &[
        VerbDictionaryEntry { lemma: "opustašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opustiti" => &[
        VerbDictionaryEntry { lemma: "opustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opustošiti" => &[
        VerbDictionaryEntry { lemma: "opustošiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opustěti" => &[
        VerbDictionaryEntry { lemma: "opustěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "opušćati" => &[
        VerbDictionaryEntry { lemma: "opušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opyliti" => &[
        VerbDictionaryEntry { lemma: "opyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "opyljati" => &[
        VerbDictionaryEntry { lemma: "opyljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "opŕti sę" => &[
        VerbDictionaryEntry { lemma: "opŕti sę", addition: "(opre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "orati" => &[
        VerbDictionaryEntry { lemma: "orati", addition: "(oŕe)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "orašati" => &[
        VerbDictionaryEntry { lemma: "orašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "organizovati" => &[
        VerbDictionaryEntry { lemma: "organizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "orientovati sę" => &[
        VerbDictionaryEntry { lemma: "orientovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "oriti" => &[
        VerbDictionaryEntry { lemma: "oriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "orkestrovati" => &[
        VerbDictionaryEntry { lemma: "orkestrovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "orositi" => &[
        VerbDictionaryEntry { lemma: "orositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "orųdovati" => &[
        VerbDictionaryEntry { lemma: "orųdovati", addition: "(+5)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(5) },
    ],
    "orųžiti" => &[
        VerbDictionaryEntry { lemma: "orųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osaditi" => &[
        VerbDictionaryEntry { lemma: "osaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osaditi sę" => &[
        VerbDictionaryEntry { lemma: "osaditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osamotiti" => &[
        VerbDictionaryEntry { lemma: "osamotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osamotiti sę" => &[
        VerbDictionaryEntry { lemma: "osamotiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osađati" => &[
        VerbDictionaryEntry { lemma: "osađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osedlati" => &[
        VerbDictionaryEntry { lemma: "osedlati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oseliti sę" => &[
        VerbDictionaryEntry { lemma: "oseliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osiliti" => &[
        VerbDictionaryEntry { lemma: "osiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osiljati" => &[
        VerbDictionaryEntry { lemma: "osiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osirotiti" => &[
        VerbDictionaryEntry { lemma: "osirotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osirotěti" => &[
        VerbDictionaryEntry { lemma: "osirotěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "osivěti" => &[
        VerbDictionaryEntry { lemma: "osivěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oskopiti" => &[
        VerbDictionaryEntry { lemma: "oskopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oskubati" => &[
        VerbDictionaryEntry { lemma: "oskubati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oskubyvati" => &[
        VerbDictionaryEntry { lemma: "oskubyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oskvŕniti" => &[
        VerbDictionaryEntry { lemma: "oskvŕniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oskvŕnjati" => &[
        VerbDictionaryEntry { lemma: "oskvŕnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oslabiti" => &[
        VerbDictionaryEntry { lemma: "oslabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oslabjati" => &[
        VerbDictionaryEntry { lemma: "oslabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oslaběti" => &[
        VerbDictionaryEntry { lemma: "oslaběti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "osloženiti" => &[
        VerbDictionaryEntry { lemma: "osloženiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osloženjati" => &[
        VerbDictionaryEntry { lemma: "osloženjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oslåditi" => &[
        VerbDictionaryEntry { lemma: "oslåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oslěpiti" => &[
        VerbDictionaryEntry { lemma: "oslěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oslěpjati" => &[
        VerbDictionaryEntry { lemma: "oslěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oslěpnųti" => &[
        VerbDictionaryEntry { lemma: "oslěpnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "osmažiti" => &[
        VerbDictionaryEntry { lemma: "osmažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osmoliti" => &[
        VerbDictionaryEntry { lemma: "osmoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osměliti" => &[
        VerbDictionaryEntry { lemma: "osměliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osměliti sę" => &[
        VerbDictionaryEntry { lemma: "osměliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osměljati" => &[
        VerbDictionaryEntry { lemma: "osměljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osměljati sę" => &[
        VerbDictionaryEntry { lemma: "osměljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "osnovati" => &[
        VerbDictionaryEntry { lemma: "osnovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osnovyvati" => &[
        VerbDictionaryEntry { lemma: "osnovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osněžiti" => &[
        VerbDictionaryEntry { lemma: "osněžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osoliti" => &[
        VerbDictionaryEntry { lemma: "osoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osparjati" => &[
        VerbDictionaryEntry { lemma: "osparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osporiti" => &[
        VerbDictionaryEntry { lemma: "osporiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osråmiti" => &[
        VerbDictionaryEntry { lemma: "osråmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ostarěti" => &[
        VerbDictionaryEntry { lemma: "ostarěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ostati" => &[
        VerbDictionaryEntry { lemma: "ostati", addition: "(ostane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ostavati" => &[
        VerbDictionaryEntry { lemma: "ostavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ostaviti" => &[
        VerbDictionaryEntry { lemma: "ostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ostavjati" => &[
        VerbDictionaryEntry { lemma: "ostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ostrigati" => &[
        VerbDictionaryEntry { lemma: "ostrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ostrigti" => &[
        VerbDictionaryEntry { lemma: "ostrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ostriti" => &[
        VerbDictionaryEntry { lemma: "ostriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ostrugati" => &[
        VerbDictionaryEntry { lemma: "ostrugati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ostrěgati" => &[
        VerbDictionaryEntry { lemma: "ostrěgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ostrěgati sę" => &[
        VerbDictionaryEntry { lemma: "ostrěgati sę", addition: "(+2)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(2) },
    ],
    "ostrěgti" => &[
        VerbDictionaryEntry { lemma: "ostrěgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ostuditi" => &[
        VerbDictionaryEntry { lemma: "ostuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osušati" => &[
        VerbDictionaryEntry { lemma: "osušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osušiti" => &[
        VerbDictionaryEntry { lemma: "osušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osvajati" => &[
        VerbDictionaryEntry { lemma: "osvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvatiti sę" => &[
        VerbDictionaryEntry { lemma: "osvatiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osvobađati" => &[
        VerbDictionaryEntry { lemma: "osvobađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvobađati sę" => &[
        VerbDictionaryEntry { lemma: "osvobađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "osvoboditi" => &[
        VerbDictionaryEntry { lemma: "osvoboditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osvoboditi sę" => &[
        VerbDictionaryEntry { lemma: "osvoboditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osvojiti" => &[
        VerbDictionaryEntry { lemma: "osvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvętiti" => &[
        VerbDictionaryEntry { lemma: "osvętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osvęćati" => &[
        VerbDictionaryEntry { lemma: "osvęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvědamjati sę" => &[
        VerbDictionaryEntry { lemma: "osvědamjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "osvědomiti sę" => &[
        VerbDictionaryEntry { lemma: "osvědomiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "osvětiti" => &[
        VerbDictionaryEntry { lemma: "osvětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osvětliti" => &[
        VerbDictionaryEntry { lemma: "osvětliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osvětljati" => &[
        VerbDictionaryEntry { lemma: "osvětljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvěćati" => &[
        VerbDictionaryEntry { lemma: "osvěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvěžati" => &[
        VerbDictionaryEntry { lemma: "osvěžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osvěžiti" => &[
        VerbDictionaryEntry { lemma: "osvěžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osědati" => &[
        VerbDictionaryEntry { lemma: "osědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "osěsti" => &[
        VerbDictionaryEntry { lemma: "osěsti", addition: "(osěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "osųditi" => &[
        VerbDictionaryEntry { lemma: "osųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osųđati" => &[
        VerbDictionaryEntry { lemma: "osųđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "osȯvrěmenniti" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmenniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "osȯvrěmennjati" => &[
        VerbDictionaryEntry { lemma: "osȯvrěmennjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oteliti sę" => &[
        VerbDictionaryEntry { lemma: "oteliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "otišati" => &[
        VerbDictionaryEntry { lemma: "otišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otišiti" => &[
        VerbDictionaryEntry { lemma: "otišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otraviti" => &[
        VerbDictionaryEntry { lemma: "otraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otravjati" => &[
        VerbDictionaryEntry { lemma: "otravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otrudniti" => &[
        VerbDictionaryEntry { lemma: "otrudniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otrudnjati" => &[
        VerbDictionaryEntry { lemma: "otrudnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otręsati" => &[
        VerbDictionaryEntry { lemma: "otręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otręsti" => &[
        VerbDictionaryEntry { lemma: "otręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otrězviti" => &[
        VerbDictionaryEntry { lemma: "otrězviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otrězvjati" => &[
        VerbDictionaryEntry { lemma: "otrězvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otrězvěti" => &[
        VerbDictionaryEntry { lemma: "otrězvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "otvarjati" => &[
        VerbDictionaryEntry { lemma: "otvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otvoriti" => &[
        VerbDictionaryEntry { lemma: "otvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otvŕděti" => &[
        VerbDictionaryEntry { lemma: "otvŕděti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "otęgčati" => &[
        VerbDictionaryEntry { lemma: "otęgčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otęgčiti" => &[
        VerbDictionaryEntry { lemma: "otęgčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otěniti" => &[
        VerbDictionaryEntry { lemma: "otěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otěnjati" => &[
        VerbDictionaryEntry { lemma: "otěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otųpiti" => &[
        VerbDictionaryEntry { lemma: "otųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "otųpjati" => &[
        VerbDictionaryEntry { lemma: "otųpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "otųpěti" => &[
        VerbDictionaryEntry { lemma: "otųpěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ovdověti" => &[
        VerbDictionaryEntry { lemma: "ovdověti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ovinųti" => &[
        VerbDictionaryEntry { lemma: "ovinųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ovivati" => &[
        VerbDictionaryEntry { lemma: "ovivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ovladnųti" => &[
        VerbDictionaryEntry { lemma: "ovladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ovladyvati" => &[
        VerbDictionaryEntry { lemma: "ovladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ovplyvniti" => &[
        VerbDictionaryEntry { lemma: "ovplyvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ovplyvnjati" => &[
        VerbDictionaryEntry { lemma: "ovplyvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ovějati" => &[
        VerbDictionaryEntry { lemma: "ovějati", addition: "(ověje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ovějivati" => &[
        VerbDictionaryEntry { lemma: "ovějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ověnčati" => &[
        VerbDictionaryEntry { lemma: "ověnčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ověnčiti" => &[
        VerbDictionaryEntry { lemma: "ověnčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ověriti" => &[
        VerbDictionaryEntry { lemma: "ověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ověrjati" => &[
        VerbDictionaryEntry { lemma: "ověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ovȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "ovȯlgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ozdabjati" => &[
        VerbDictionaryEntry { lemma: "ozdabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ozdobiti" => &[
        VerbDictionaryEntry { lemma: "ozdobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ozdravjati" => &[
        VerbDictionaryEntry { lemma: "ozdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ozdråviti" => &[
        VerbDictionaryEntry { lemma: "ozdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ozdråvěti" => &[
        VerbDictionaryEntry { lemma: "ozdråvěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ozeleniti" => &[
        VerbDictionaryEntry { lemma: "ozeleniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ozelenjati" => &[
        VerbDictionaryEntry { lemma: "ozelenjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ozlobiti" => &[
        VerbDictionaryEntry { lemma: "ozlobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ozlobiti sę" => &[
        VerbDictionaryEntry { lemma: "ozlobiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "oznamenovati" => &[
        VerbDictionaryEntry { lemma: "oznamenovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oznamenovyvati" => &[
        VerbDictionaryEntry { lemma: "oznamenovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "oznanjati" => &[
        VerbDictionaryEntry { lemma: "oznanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "označati" => &[
        VerbDictionaryEntry { lemma: "označati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "označiti" => &[
        VerbDictionaryEntry { lemma: "označiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ozvěrěti" => &[
        VerbDictionaryEntry { lemma: "ozvěrěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ozębti" => &[
        VerbDictionaryEntry { lemma: "ozębti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "očariti" => &[
        VerbDictionaryEntry { lemma: "očariti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "očarovati" => &[
        VerbDictionaryEntry { lemma: "očarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "očarovati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: Some(3) },
    ],
    "očarovyvati" => &[
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "očarovyvati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(3) },
    ],
    "očekyvati" => &[
        VerbDictionaryEntry { lemma: "očekyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "očistiti" => &[
        VerbDictionaryEntry { lemma: "očistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "očišćati" => &[
        VerbDictionaryEntry { lemma: "očišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "očrkati" => &[
        VerbDictionaryEntry { lemma: "očrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "očrknųti" => &[
        VerbDictionaryEntry { lemma: "očrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "očrniti" => &[
        VerbDictionaryEntry { lemma: "očrniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "očrnjati" => &[
        VerbDictionaryEntry { lemma: "očrnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ošalěti" => &[
        VerbDictionaryEntry { lemma: "ošalěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oščeniti sę" => &[
        VerbDictionaryEntry { lemma: "oščeniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "oženiti" => &[
        VerbDictionaryEntry { lemma: "oženiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oženiti sę" => &[
        VerbDictionaryEntry { lemma: "oženiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "ožiti" => &[
        VerbDictionaryEntry { lemma: "ožiti", addition: "(ožive)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "oživati" => &[
        VerbDictionaryEntry { lemma: "oživati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "oživiti" => &[
        VerbDictionaryEntry { lemma: "oživiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "oživjati" => &[
        VerbDictionaryEntry { lemma: "oživjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ožrěbiti sę" => &[
        VerbDictionaryEntry { lemma: "ožrěbiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "ožȯltěti" => &[
        VerbDictionaryEntry { lemma: "ožȯltěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "padati" => &[
        VerbDictionaryEntry { lemma: "padati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pakovati" => &[
        VerbDictionaryEntry { lemma: "pakovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "palatalizovati" => &[
        VerbDictionaryEntry { lemma: "palatalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "paliti" => &[
        VerbDictionaryEntry { lemma: "paliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pamętati" => &[
        VerbDictionaryEntry { lemma: "pamętati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "panikovati" => &[
        VerbDictionaryEntry { lemma: "panikovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "paralizovati" => &[
        VerbDictionaryEntry { lemma: "paralizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "parazitovati" => &[
        VerbDictionaryEntry { lemma: "parazitovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "parfumovati" => &[
        VerbDictionaryEntry { lemma: "parfumovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "parfumovati sę" => &[
        VerbDictionaryEntry { lemma: "parfumovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "pariti" => &[
        VerbDictionaryEntry { lemma: "pariti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "parkovati" => &[
        VerbDictionaryEntry { lemma: "parkovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "parkovati sę" => &[
        VerbDictionaryEntry { lemma: "parkovati sę", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pasti" => &[
        VerbDictionaryEntry { lemma: "pasti", addition: "(pade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "pasti", addition: "(pase)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pasti sę" => &[
        VerbDictionaryEntry { lemma: "pasti sę", addition: "(pase)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "patronizovati" => &[
        VerbDictionaryEntry { lemma: "patronizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pauzovati" => &[
        VerbDictionaryEntry { lemma: "pauzovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pekti" => &[
        VerbDictionaryEntry { lemma: "pekti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pekti sę" => &[
        VerbDictionaryEntry { lemma: "pekti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "pečatati" => &[
        VerbDictionaryEntry { lemma: "pečatati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "phati" => &[
        VerbDictionaryEntry { lemma: "phati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "phnųti" => &[
        VerbDictionaryEntry { lemma: "phnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pikirovati" => &[
        VerbDictionaryEntry { lemma: "pikirovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "piliti" => &[
        VerbDictionaryEntry { lemma: "piliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pinati" => &[
        VerbDictionaryEntry { lemma: "pinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pirovati" => &[
        VerbDictionaryEntry { lemma: "pirovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pisati" => &[
        VerbDictionaryEntry { lemma: "pisati", addition: "(piše)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "piskati" => &[
        VerbDictionaryEntry { lemma: "piskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "piti" => &[
        VerbDictionaryEntry { lemma: "piti", addition: "(pije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pišati" => &[
        VerbDictionaryEntry { lemma: "pišati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "piščeti" => &[
        VerbDictionaryEntry { lemma: "piščeti", addition: "(pišče)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "plakati" => &[
        VerbDictionaryEntry { lemma: "plakati", addition: "(plače)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "planovati" => &[
        VerbDictionaryEntry { lemma: "planovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "platiti" => &[
        VerbDictionaryEntry { lemma: "platiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "plavati" => &[
        VerbDictionaryEntry { lemma: "plavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pleskati" => &[
        VerbDictionaryEntry { lemma: "pleskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "plesknųti" => &[
        VerbDictionaryEntry { lemma: "plesknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "plesti" => &[
        VerbDictionaryEntry { lemma: "plesti", addition: "(plete)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pljunųti" => &[
        VerbDictionaryEntry { lemma: "pljunųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pljuskati" => &[
        VerbDictionaryEntry { lemma: "pljuskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pljusknųti" => &[
        VerbDictionaryEntry { lemma: "pljusknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pljuvati" => &[
        VerbDictionaryEntry { lemma: "pljuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ploditi" => &[
        VerbDictionaryEntry { lemma: "ploditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pluti" => &[
        VerbDictionaryEntry { lemma: "pluti", addition: "(plove)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "plyvti" => &[
        VerbDictionaryEntry { lemma: "plyvti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "plęsati" => &[
        VerbDictionaryEntry { lemma: "plęsati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "plěniti" => &[
        VerbDictionaryEntry { lemma: "plěniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "plěti" => &[
        VerbDictionaryEntry { lemma: "plěti", addition: "(plěje/plěve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pobiti" => &[
        VerbDictionaryEntry { lemma: "pobiti", addition: "(pobije)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poblågodariti" => &[
        VerbDictionaryEntry { lemma: "poblågodariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poblågoželati" => &[
        VerbDictionaryEntry { lemma: "poblågoželati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "pobuditi" => &[
        VerbDictionaryEntry { lemma: "pobuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pobuđati" => &[
        VerbDictionaryEntry { lemma: "pobuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poběditi" => &[
        VerbDictionaryEntry { lemma: "poběditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poběgti" => &[
        VerbDictionaryEntry { lemma: "poběgti", addition: "(poběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poběđati" => &[
        VerbDictionaryEntry { lemma: "poběđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pocělovati" => &[
        VerbDictionaryEntry { lemma: "pocělovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "podariti" => &[
        VerbDictionaryEntry { lemma: "podariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podati" => &[
        VerbDictionaryEntry { lemma: "podati", addition: "(poda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podavati" => &[
        VerbDictionaryEntry { lemma: "podavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podbirati" => &[
        VerbDictionaryEntry { lemma: "podbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podbrati" => &[
        VerbDictionaryEntry { lemma: "podbrati", addition: "(podbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poddati" => &[
        VerbDictionaryEntry { lemma: "poddati", addition: "(podda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poddati sę" => &[
        VerbDictionaryEntry { lemma: "poddati sę", addition: "(podda)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poddavati" => &[
        VerbDictionaryEntry { lemma: "poddavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poddavati sę" => &[
        VerbDictionaryEntry { lemma: "poddavati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "poddŕžati" => &[
        VerbDictionaryEntry { lemma: "poddŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poddŕživati" => &[
        VerbDictionaryEntry { lemma: "poddŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podględati" => &[
        VerbDictionaryEntry { lemma: "podględati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "podględěti" => &[
        VerbDictionaryEntry { lemma: "podględěti", addition: "(podględi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "podgovarjati" => &[
        VerbDictionaryEntry { lemma: "podgovarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podgovoriti" => &[
        VerbDictionaryEntry { lemma: "podgovoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podgrěti" => &[
        VerbDictionaryEntry { lemma: "podgrěti", addition: "(podgrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podgrěvati" => &[
        VerbDictionaryEntry { lemma: "podgrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podhoditi" => &[
        VerbDictionaryEntry { lemma: "podhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "podimati" => &[
        VerbDictionaryEntry { lemma: "podimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podjęti" => &[
        VerbDictionaryEntry { lemma: "podjęti", addition: "(podȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podkladati" => &[
        VerbDictionaryEntry { lemma: "podkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podključati" => &[
        VerbDictionaryEntry { lemma: "podključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podključati sę" => &[
        VerbDictionaryEntry { lemma: "podključati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "podključiti" => &[
        VerbDictionaryEntry { lemma: "podključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podključiti sę" => &[
        VerbDictionaryEntry { lemma: "podključiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "podkopati" => &[
        VerbDictionaryEntry { lemma: "podkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podkopyvati" => &[
        VerbDictionaryEntry { lemma: "podkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podkovati" => &[
        VerbDictionaryEntry { lemma: "podkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podkovyvati" => &[
        VerbDictionaryEntry { lemma: "podkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podkrěpiti" => &[
        VerbDictionaryEntry { lemma: "podkrěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podkrěpjati" => &[
        VerbDictionaryEntry { lemma: "podkrěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podkupiti" => &[
        VerbDictionaryEntry { lemma: "podkupiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podkupovati" => &[
        VerbDictionaryEntry { lemma: "podkupovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podlagati" => &[
        VerbDictionaryEntry { lemma: "podlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podležati" => &[
        VerbDictionaryEntry { lemma: "podležati", addition: "(podleži) (+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "podliti" => &[
        VerbDictionaryEntry { lemma: "podliti", addition: "(podlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podlivati" => &[
        VerbDictionaryEntry { lemma: "podlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podložiti" => &[
        VerbDictionaryEntry { lemma: "podložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podměniti" => &[
        VerbDictionaryEntry { lemma: "podměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podměnjati" => &[
        VerbDictionaryEntry { lemma: "podměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podnurjati" => &[
        VerbDictionaryEntry { lemma: "podnurjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podobati sę" => &[
        VerbDictionaryEntry { lemma: "podobati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "podpaliti" => &[
        VerbDictionaryEntry { lemma: "podpaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podpaljati" => &[
        VerbDictionaryEntry { lemma: "podpaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podpirati" => &[
        VerbDictionaryEntry { lemma: "podpirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podpisati" => &[
        VerbDictionaryEntry { lemma: "podpisati", addition: "(podpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podpisyvati" => &[
        VerbDictionaryEntry { lemma: "podpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podporiti" => &[
        VerbDictionaryEntry { lemma: "podporiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podružiti sę" => &[
        VerbDictionaryEntry { lemma: "podružiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "podråzděliti" => &[
        VerbDictionaryEntry { lemma: "podråzděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podråzděljati" => &[
        VerbDictionaryEntry { lemma: "podråzděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podręditi" => &[
        VerbDictionaryEntry { lemma: "podręditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podręđati" => &[
        VerbDictionaryEntry { lemma: "podręđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podslušati" => &[
        VerbDictionaryEntry { lemma: "podslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podslušivati" => &[
        VerbDictionaryEntry { lemma: "podslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podsměhati sę" => &[
        VerbDictionaryEntry { lemma: "podsměhati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "podsměhnųti sę" => &[
        VerbDictionaryEntry { lemma: "podsměhnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "podstrěkati" => &[
        VerbDictionaryEntry { lemma: "podstrěkati", addition: "(podstrěče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podstrěknųti" => &[
        VerbDictionaryEntry { lemma: "podstrěknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podtiskati" => &[
        VerbDictionaryEntry { lemma: "podtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podtisknųti" => &[
        VerbDictionaryEntry { lemma: "podtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podtrimati" => &[
        VerbDictionaryEntry { lemma: "podtrimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podtrimyvati" => &[
        VerbDictionaryEntry { lemma: "podtrimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podvajati" => &[
        VerbDictionaryEntry { lemma: "podvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podvažati" => &[
        VerbDictionaryEntry { lemma: "podvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podvažiti" => &[
        VerbDictionaryEntry { lemma: "podvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podvisiti" => &[
        VerbDictionaryEntry { lemma: "podvisiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podvojiti" => &[
        VerbDictionaryEntry { lemma: "podvojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podvŕgati" => &[
        VerbDictionaryEntry { lemma: "podvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "podvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podzirati" => &[
        VerbDictionaryEntry { lemma: "podzirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podzrěti" => &[
        VerbDictionaryEntry { lemma: "podzrěti", addition: "(podzri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podzrěvati" => &[
        VerbDictionaryEntry { lemma: "podzrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podčrkati" => &[
        VerbDictionaryEntry { lemma: "podčrkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "podčrknųti" => &[
        VerbDictionaryEntry { lemma: "podčrknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "podękovati" => &[
        VerbDictionaryEntry { lemma: "podękovati", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "poděliti" => &[
        VerbDictionaryEntry { lemma: "poděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poděliti sę" => &[
        VerbDictionaryEntry { lemma: "poděliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poděti sę" => &[
        VerbDictionaryEntry { lemma: "poděti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poděvati sę" => &[
        VerbDictionaryEntry { lemma: "poděvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "podȯjdti" => &[
        VerbDictionaryEntry { lemma: "podȯjdti", addition: "(podȯjde; podšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pogaršati" => &[
        VerbDictionaryEntry { lemma: "pogaršati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pogladiti" => &[
        VerbDictionaryEntry { lemma: "pogladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poględati" => &[
        VerbDictionaryEntry { lemma: "poględati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poględnųti" => &[
        VerbDictionaryEntry { lemma: "poględnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poględyvati" => &[
        VerbDictionaryEntry { lemma: "poględyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "poglųbiti" => &[
        VerbDictionaryEntry { lemma: "poglųbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poglųbjati" => &[
        VerbDictionaryEntry { lemma: "poglųbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pogoršiti" => &[
        VerbDictionaryEntry { lemma: "pogoršiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pogovoriti" => &[
        VerbDictionaryEntry { lemma: "pogovoriti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pogrebati" => &[
        VerbDictionaryEntry { lemma: "pogrebati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pogrebti" => &[
        VerbDictionaryEntry { lemma: "pogrebti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pogrěšiti" => &[
        VerbDictionaryEntry { lemma: "pogrěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pogrųziti" => &[
        VerbDictionaryEntry { lemma: "pogrųziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pogrųžati" => &[
        VerbDictionaryEntry { lemma: "pogrųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pogynati" => &[
        VerbDictionaryEntry { lemma: "pogynati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pogynųti" => &[
        VerbDictionaryEntry { lemma: "pogynųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pogȯltnųti" => &[
        VerbDictionaryEntry { lemma: "pogȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pohristijaniti" => &[
        VerbDictionaryEntry { lemma: "pohristijaniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pohristijanjati" => &[
        VerbDictionaryEntry { lemma: "pohristijanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pohvaliti" => &[
        VerbDictionaryEntry { lemma: "pohvaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pohytiti" => &[
        VerbDictionaryEntry { lemma: "pohytiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pohyćati" => &[
        VerbDictionaryEntry { lemma: "pohyćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poiskati" => &[
        VerbDictionaryEntry { lemma: "poiskati", addition: "(poišče)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poiskyvati" => &[
        VerbDictionaryEntry { lemma: "poiskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pojaviti sę" => &[
        VerbDictionaryEntry { lemma: "pojaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pojavjati sę" => &[
        VerbDictionaryEntry { lemma: "pojavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "pojdti" => &[
        VerbDictionaryEntry { lemma: "pojdti", addition: "(pojde; pošėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pojehati" => &[
        VerbDictionaryEntry { lemma: "pojehati", addition: "(pojede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pojiti" => &[
        VerbDictionaryEntry { lemma: "pojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pojmati" => &[
        VerbDictionaryEntry { lemma: "pojmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pojęti" => &[
        VerbDictionaryEntry { lemma: "pojęti", addition: "(pojme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokajati sę" => &[
        VerbDictionaryEntry { lemma: "pokajati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pokarati" => &[
        VerbDictionaryEntry { lemma: "pokarati", addition: "(pokare)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "pokarati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokarjati" => &[
        VerbDictionaryEntry { lemma: "pokarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pokazati" => &[
        VerbDictionaryEntry { lemma: "pokazati", addition: "(pokaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokazyvati" => &[
        VerbDictionaryEntry { lemma: "pokazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pokladati" => &[
        VerbDictionaryEntry { lemma: "pokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poklicati" => &[
        VerbDictionaryEntry { lemma: "poklicati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pokloniti sę" => &[
        VerbDictionaryEntry { lemma: "pokloniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pokoriti" => &[
        VerbDictionaryEntry { lemma: "pokoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokositi" => &[
        VerbDictionaryEntry { lemma: "pokositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokryti" => &[
        VerbDictionaryEntry { lemma: "pokryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokryvati" => &[
        VerbDictionaryEntry { lemma: "pokryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pokrėstiti" => &[
        VerbDictionaryEntry { lemma: "pokrėstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokusiti" => &[
        VerbDictionaryEntry { lemma: "pokusiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pokušati" => &[
        VerbDictionaryEntry { lemma: "pokušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "polakovati" => &[
        VerbDictionaryEntry { lemma: "polakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poletěti" => &[
        VerbDictionaryEntry { lemma: "poletěti", addition: "(poleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "polirati" => &[
        VerbDictionaryEntry { lemma: "polirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "politi" => &[
        VerbDictionaryEntry { lemma: "politi", addition: "(polije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "polivati" => &[
        VerbDictionaryEntry { lemma: "polivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poljzovati" => &[
        VerbDictionaryEntry { lemma: "poljzovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "položiti" => &[
        VerbDictionaryEntry { lemma: "položiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "polučati" => &[
        VerbDictionaryEntry { lemma: "polučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "polučiti" => &[
        VerbDictionaryEntry { lemma: "polučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "polězti" => &[
        VerbDictionaryEntry { lemma: "polězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pomagati" => &[
        VerbDictionaryEntry { lemma: "pomagati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "pomazati" => &[
        VerbDictionaryEntry { lemma: "pomazati", addition: "(pomaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pomilovati" => &[
        VerbDictionaryEntry { lemma: "pomilovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pomiriti" => &[
        VerbDictionaryEntry { lemma: "pomiriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pomněti" => &[
        VerbDictionaryEntry { lemma: "pomněti", addition: "(pomni)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pomogti" => &[
        VerbDictionaryEntry { lemma: "pomogti", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "pomyliti" => &[
        VerbDictionaryEntry { lemma: "pomyliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pomyliti sę" => &[
        VerbDictionaryEntry { lemma: "pomyliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pomysliti" => &[
        VerbDictionaryEntry { lemma: "pomysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poměstiti" => &[
        VerbDictionaryEntry { lemma: "poměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poměšćati" => &[
        VerbDictionaryEntry { lemma: "poměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poniziti" => &[
        VerbDictionaryEntry { lemma: "poniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ponižati" => &[
        VerbDictionaryEntry { lemma: "ponižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "popiti" => &[
        VerbDictionaryEntry { lemma: "popiti", addition: "(popije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "popivati" => &[
        VerbDictionaryEntry { lemma: "popivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poplaviti" => &[
        VerbDictionaryEntry { lemma: "poplaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poplavjati" => &[
        VerbDictionaryEntry { lemma: "poplavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "popluti" => &[
        VerbDictionaryEntry { lemma: "popluti", addition: "(poplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poplyvti" => &[
        VerbDictionaryEntry { lemma: "poplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "popraviti" => &[
        VerbDictionaryEntry { lemma: "popraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "popravjati" => &[
        VerbDictionaryEntry { lemma: "popravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poprobovati" => &[
        VerbDictionaryEntry { lemma: "poprobovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poprositi" => &[
        VerbDictionaryEntry { lemma: "poprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "popularizovati" => &[
        VerbDictionaryEntry { lemma: "popularizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "popustiti" => &[
        VerbDictionaryEntry { lemma: "popustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "popušćati" => &[
        VerbDictionaryEntry { lemma: "popušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "popȯlzati" => &[
        VerbDictionaryEntry { lemma: "popȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "popȯlzti" => &[
        VerbDictionaryEntry { lemma: "popȯlzti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poraditi" => &[
        VerbDictionaryEntry { lemma: "poraditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poraniti" => &[
        VerbDictionaryEntry { lemma: "poraniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poraziti" => &[
        VerbDictionaryEntry { lemma: "poraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "porađati" => &[
        VerbDictionaryEntry { lemma: "porađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poražati" => &[
        VerbDictionaryEntry { lemma: "poražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poroditi" => &[
        VerbDictionaryEntry { lemma: "poroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "porušiti" => &[
        VerbDictionaryEntry { lemma: "porušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poråbiti" => &[
        VerbDictionaryEntry { lemma: "poråbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poråbovati" => &[
        VerbDictionaryEntry { lemma: "poråbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poråsti" => &[
        VerbDictionaryEntry { lemma: "poråsti", addition: "(poråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poråvniti" => &[
        VerbDictionaryEntry { lemma: "poråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poråzuměti sę" => &[
        VerbDictionaryEntry { lemma: "poråzuměti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poråzuměvati sę" => &[
        VerbDictionaryEntry { lemma: "poråzuměvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "porųbati" => &[
        VerbDictionaryEntry { lemma: "porųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "porųčati sę" => &[
        VerbDictionaryEntry { lemma: "porųčati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "porųčiti sę" => &[
        VerbDictionaryEntry { lemma: "porųčiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poscati sę" => &[
        VerbDictionaryEntry { lemma: "poscati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poskųpiti sę" => &[
        VerbDictionaryEntry { lemma: "poskųpiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poslati" => &[
        VerbDictionaryEntry { lemma: "poslati", addition: "(pošlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poslizgnųti sę" => &[
        VerbDictionaryEntry { lemma: "poslizgnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poslušati" => &[
        VerbDictionaryEntry { lemma: "poslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posluživati sę" => &[
        VerbDictionaryEntry { lemma: "posluživati sę", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(5) },
    ],
    "posoliti" => &[
        VerbDictionaryEntry { lemma: "posoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pospati" => &[
        VerbDictionaryEntry { lemma: "pospati", addition: "(pospi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pospěšiti" => &[
        VerbDictionaryEntry { lemma: "pospěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "posrěbriti" => &[
        VerbDictionaryEntry { lemma: "posrěbriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posrěbrjati" => &[
        VerbDictionaryEntry { lemma: "posrěbrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posrědkovati" => &[
        VerbDictionaryEntry { lemma: "posrědkovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "postanavjati" => &[
        VerbDictionaryEntry { lemma: "postanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "postanoviti" => &[
        VerbDictionaryEntry { lemma: "postanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postarati sę" => &[
        VerbDictionaryEntry { lemma: "postarati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "postarěti" => &[
        VerbDictionaryEntry { lemma: "postarěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "postaviti" => &[
        VerbDictionaryEntry { lemma: "postaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postavjati" => &[
        VerbDictionaryEntry { lemma: "postavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posteliti" => &[
        VerbDictionaryEntry { lemma: "posteliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postigati" => &[
        VerbDictionaryEntry { lemma: "postigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "postignųti" => &[
        VerbDictionaryEntry { lemma: "postignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postlati" => &[
        VerbDictionaryEntry { lemma: "postlati", addition: "(postelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postulovati" => &[
        VerbDictionaryEntry { lemma: "postulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "postųpati" => &[
        VerbDictionaryEntry { lemma: "postųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "postųpiti" => &[
        VerbDictionaryEntry { lemma: "postųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "posvętiti" => &[
        VerbDictionaryEntry { lemma: "posvętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posvęćati" => &[
        VerbDictionaryEntry { lemma: "posvęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posvědčati" => &[
        VerbDictionaryEntry { lemma: "posvědčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posvědčiti" => &[
        VerbDictionaryEntry { lemma: "posvědčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "posvědčiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "posylati" => &[
        VerbDictionaryEntry { lemma: "posylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posědati" => &[
        VerbDictionaryEntry { lemma: "posědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posějati" => &[
        VerbDictionaryEntry { lemma: "posějati", addition: "(posěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posěkati" => &[
        VerbDictionaryEntry { lemma: "posěkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posěkti" => &[
        VerbDictionaryEntry { lemma: "posěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posěsti" => &[
        VerbDictionaryEntry { lemma: "posěsti", addition: "(posěde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posětiti" => &[
        VerbDictionaryEntry { lemma: "posětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "posěćati" => &[
        VerbDictionaryEntry { lemma: "posěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "posȯvětovati" => &[
        VerbDictionaryEntry { lemma: "posȯvětovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "potapjati" => &[
        VerbDictionaryEntry { lemma: "potapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "potelefonovati" => &[
        VerbDictionaryEntry { lemma: "potelefonovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "potiti sę" => &[
        VerbDictionaryEntry { lemma: "potiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "potopiti" => &[
        VerbDictionaryEntry { lemma: "potopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "potrajati" => &[
        VerbDictionaryEntry { lemma: "potrajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "potrvati" => &[
        VerbDictionaryEntry { lemma: "potrvati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "potręsati" => &[
        VerbDictionaryEntry { lemma: "potręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "potręsti" => &[
        VerbDictionaryEntry { lemma: "potręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "potrěbovati" => &[
        VerbDictionaryEntry { lemma: "potrěbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "potvŕditi" => &[
        VerbDictionaryEntry { lemma: "potvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "potvŕđati" => &[
        VerbDictionaryEntry { lemma: "potvŕđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "potėmněti" => &[
        VerbDictionaryEntry { lemma: "potėmněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "potěti" => &[
        VerbDictionaryEntry { lemma: "potěti", addition: "(poti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "poučati" => &[
        VerbDictionaryEntry { lemma: "poučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "poučiti" => &[
        VerbDictionaryEntry { lemma: "poučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povaliti" => &[
        VerbDictionaryEntry { lemma: "povaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "považati" => &[
        VerbDictionaryEntry { lemma: "považati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "považiti" => &[
        VerbDictionaryEntry { lemma: "považiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poveseliti sę" => &[
        VerbDictionaryEntry { lemma: "poveseliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "povraćati" => &[
        VerbDictionaryEntry { lemma: "povraćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "povråtiti" => &[
        VerbDictionaryEntry { lemma: "povråtiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povstati" => &[
        VerbDictionaryEntry { lemma: "povstati", addition: "(povstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "povstavati" => &[
        VerbDictionaryEntry { lemma: "povstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "povtarjati" => &[
        VerbDictionaryEntry { lemma: "povtarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "povtoriti" => &[
        VerbDictionaryEntry { lemma: "povtoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povysiti" => &[
        VerbDictionaryEntry { lemma: "povysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povyšati" => &[
        VerbDictionaryEntry { lemma: "povyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "povęzati" => &[
        VerbDictionaryEntry { lemma: "povęzati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povęzyvati" => &[
        VerbDictionaryEntry { lemma: "povęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "povědati" => &[
        VerbDictionaryEntry { lemma: "povědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pověděti" => &[
        VerbDictionaryEntry { lemma: "pověděti", addition: "(pově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pověriti" => &[
        VerbDictionaryEntry { lemma: "pověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pověrjati" => &[
        VerbDictionaryEntry { lemma: "pověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pověsiti" => &[
        VerbDictionaryEntry { lemma: "pověsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povŕnųti" => &[
        VerbDictionaryEntry { lemma: "povŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "povŕtati" => &[
        VerbDictionaryEntry { lemma: "povŕtati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozajmati" => &[
        VerbDictionaryEntry { lemma: "pozajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozajęti" => &[
        VerbDictionaryEntry { lemma: "pozajęti", addition: "(pozajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pozastaviti" => &[
        VerbDictionaryEntry { lemma: "pozastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pozdravjati" => &[
        VerbDictionaryEntry { lemma: "pozdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozdråviti" => &[
        VerbDictionaryEntry { lemma: "pozdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pozlaćati" => &[
        VerbDictionaryEntry { lemma: "pozlaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozlåtiti" => &[
        VerbDictionaryEntry { lemma: "pozlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poznati" => &[
        VerbDictionaryEntry { lemma: "poznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poznavati" => &[
        VerbDictionaryEntry { lemma: "poznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozovati" => &[
        VerbDictionaryEntry { lemma: "pozovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pozvaljati" => &[
        VerbDictionaryEntry { lemma: "pozvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pozvati" => &[
        VerbDictionaryEntry { lemma: "pozvati", addition: "(pozȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pozvoliti" => &[
        VerbDictionaryEntry { lemma: "pozvoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pozvoniti" => &[
        VerbDictionaryEntry { lemma: "pozvoniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "počekati" => &[
        VerbDictionaryEntry { lemma: "počekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "počinati" => &[
        VerbDictionaryEntry { lemma: "počinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "počinati sę" => &[
        VerbDictionaryEntry { lemma: "počinati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "počrveněti" => &[
        VerbDictionaryEntry { lemma: "počrveněti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "počęti" => &[
        VerbDictionaryEntry { lemma: "počęti", addition: "(počne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "počęti sę" => &[
        VerbDictionaryEntry { lemma: "počęti sę", addition: "(počne)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "poškoditi" => &[
        VerbDictionaryEntry { lemma: "poškoditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "poščęditi" => &[
        VerbDictionaryEntry { lemma: "poščęditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "požaliti" => &[
        VerbDictionaryEntry { lemma: "požaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "poželati" => &[
        VerbDictionaryEntry { lemma: "poželati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "požirati" => &[
        VerbDictionaryEntry { lemma: "požirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "požrtvovati" => &[
        VerbDictionaryEntry { lemma: "požrtvovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "požrěti" => &[
        VerbDictionaryEntry { lemma: "požrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "požędati" => &[
        VerbDictionaryEntry { lemma: "požędati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "požęti" => &[
        VerbDictionaryEntry { lemma: "požęti", addition: "(požne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pracovati" => &[
        VerbDictionaryEntry { lemma: "pracovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "praktikovati" => &[
        VerbDictionaryEntry { lemma: "praktikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prati" => &[
        VerbDictionaryEntry { lemma: "prati", addition: "(pere)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "praviti" => &[
        VerbDictionaryEntry { lemma: "praviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prašćati" => &[
        VerbDictionaryEntry { lemma: "prašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prašćati sę" => &[
        VerbDictionaryEntry { lemma: "prašćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "preferovati" => &[
        VerbDictionaryEntry { lemma: "preferovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pribiti" => &[
        VerbDictionaryEntry { lemma: "pribiti", addition: "(pribije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pribivati" => &[
        VerbDictionaryEntry { lemma: "pribivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "približati sę" => &[
        VerbDictionaryEntry { lemma: "približati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "približiti sę" => &[
        VerbDictionaryEntry { lemma: "približiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pribyti" => &[
        VerbDictionaryEntry { lemma: "pribyti", addition: "(pribųde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pribyvati" => &[
        VerbDictionaryEntry { lemma: "pribyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pricěliti" => &[
        VerbDictionaryEntry { lemma: "pricěliti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pridati" => &[
        VerbDictionaryEntry { lemma: "pridati", addition: "(prida)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pridavati" => &[
        VerbDictionaryEntry { lemma: "pridavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pridumati" => &[
        VerbDictionaryEntry { lemma: "pridumati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pridělati" => &[
        VerbDictionaryEntry { lemma: "pridělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priděliti" => &[
        VerbDictionaryEntry { lemma: "priděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priděljati" => &[
        VerbDictionaryEntry { lemma: "priděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pridŕživati sę" => &[
        VerbDictionaryEntry { lemma: "pridŕživati sę", addition: "(+2)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(2) },
    ],
    "prigađati sę" => &[
        VerbDictionaryEntry { lemma: "prigađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "priględati" => &[
        VerbDictionaryEntry { lemma: "priględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "priględati sę" => &[
        VerbDictionaryEntry { lemma: "priględati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "priględěti" => &[
        VerbDictionaryEntry { lemma: "priględěti", addition: "(priględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priględěti sę" => &[
        VerbDictionaryEntry { lemma: "priględěti sę", addition: "(priględi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prignųti" => &[
        VerbDictionaryEntry { lemma: "prignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prigoditi sę" => &[
        VerbDictionaryEntry { lemma: "prigoditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prigotoviti" => &[
        VerbDictionaryEntry { lemma: "prigotoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prigybati" => &[
        VerbDictionaryEntry { lemma: "prigybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prihoditi" => &[
        VerbDictionaryEntry { lemma: "prihoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prijateliti sę" => &[
        VerbDictionaryEntry { lemma: "prijateliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prijati" => &[
        VerbDictionaryEntry { lemma: "prijati", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(3) },
    ],
    "prijaviti sę" => &[
        VerbDictionaryEntry { lemma: "prijaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prijavjati sę" => &[
        VerbDictionaryEntry { lemma: "prijavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prijdti" => &[
        VerbDictionaryEntry { lemma: "prijdti", addition: "(prijde; prišėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prijehati" => &[
        VerbDictionaryEntry { lemma: "prijehati", addition: "(prijede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "priježđati" => &[
        VerbDictionaryEntry { lemma: "priježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prijmati" => &[
        VerbDictionaryEntry { lemma: "prijmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prijmati sę" => &[
        VerbDictionaryEntry { lemma: "prijmati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prijęti" => &[
        VerbDictionaryEntry { lemma: "prijęti", addition: "(prijme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prijęti sę" => &[
        VerbDictionaryEntry { lemma: "prijęti sę", addition: "(prijme)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prikazati" => &[
        VerbDictionaryEntry { lemma: "prikazati", addition: "(prikaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prikazyvati" => &[
        VerbDictionaryEntry { lemma: "prikazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prikladati" => &[
        VerbDictionaryEntry { lemma: "prikladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "priletěti" => &[
        VerbDictionaryEntry { lemma: "priletěti", addition: "(prilěti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "priložiti" => &[
        VerbDictionaryEntry { lemma: "priložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prilěpiti" => &[
        VerbDictionaryEntry { lemma: "prilěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prilěpiti sę" => &[
        VerbDictionaryEntry { lemma: "prilěpiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prilětati" => &[
        VerbDictionaryEntry { lemma: "prilětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prilųčati" => &[
        VerbDictionaryEntry { lemma: "prilųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prilųčati sę" => &[
        VerbDictionaryEntry { lemma: "prilųčati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prilųčiti" => &[
        VerbDictionaryEntry { lemma: "prilųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prilųčiti sę" => &[
        VerbDictionaryEntry { lemma: "prilųčiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "priměniti" => &[
        VerbDictionaryEntry { lemma: "priměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priměnjati" => &[
        VerbDictionaryEntry { lemma: "priměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "primětiti" => &[
        VerbDictionaryEntry { lemma: "primětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priměćati" => &[
        VerbDictionaryEntry { lemma: "priměćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prinaležati" => &[
        VerbDictionaryEntry { lemma: "prinaležati", addition: "(prinaleži)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prinesti" => &[
        VerbDictionaryEntry { lemma: "prinesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prinositi" => &[
        VerbDictionaryEntry { lemma: "prinositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prinuditi" => &[
        VerbDictionaryEntry { lemma: "prinuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prinuđati" => &[
        VerbDictionaryEntry { lemma: "prinuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripadati" => &[
        VerbDictionaryEntry { lemma: "pripadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pripasti" => &[
        VerbDictionaryEntry { lemma: "pripasti", addition: "(pripade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pripinati" => &[
        VerbDictionaryEntry { lemma: "pripinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripisati" => &[
        VerbDictionaryEntry { lemma: "pripisati", addition: "(pripiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pripisyvati" => &[
        VerbDictionaryEntry { lemma: "pripisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripluti" => &[
        VerbDictionaryEntry { lemma: "pripluti", addition: "(priplove)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "priplyvati" => &[
        VerbDictionaryEntry { lemma: "priplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "priplyvti" => &[
        VerbDictionaryEntry { lemma: "priplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pripominati" => &[
        VerbDictionaryEntry { lemma: "pripominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripomněti" => &[
        VerbDictionaryEntry { lemma: "pripomněti", addition: "(pripomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pripraviti" => &[
        VerbDictionaryEntry { lemma: "pripraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pripravjati" => &[
        VerbDictionaryEntry { lemma: "pripravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripustiti" => &[
        VerbDictionaryEntry { lemma: "pripustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pripušćati" => &[
        VerbDictionaryEntry { lemma: "pripušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pripęti" => &[
        VerbDictionaryEntry { lemma: "pripęti", addition: "(pripne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisjediniti" => &[
        VerbDictionaryEntry { lemma: "prisjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisjediniti sę" => &[
        VerbDictionaryEntry { lemma: "prisjediniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prisjedinjati" => &[
        VerbDictionaryEntry { lemma: "prisjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prisjedinjati sę" => &[
        VerbDictionaryEntry { lemma: "prisjedinjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prislati" => &[
        VerbDictionaryEntry { lemma: "prislati", addition: "(prišlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisposobiti" => &[
        VerbDictionaryEntry { lemma: "prisposobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisposobjati" => &[
        VerbDictionaryEntry { lemma: "prisposobjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prispěti" => &[
        VerbDictionaryEntry { lemma: "prispěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prispěvati" => &[
        VerbDictionaryEntry { lemma: "prispěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pristojati" => &[
        VerbDictionaryEntry { lemma: "pristojati", addition: "(pristoji)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pristrigati" => &[
        VerbDictionaryEntry { lemma: "pristrigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pristrigti" => &[
        VerbDictionaryEntry { lemma: "pristrigti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisvajati" => &[
        VerbDictionaryEntry { lemma: "prisvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prisvojiti" => &[
        VerbDictionaryEntry { lemma: "prisvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisylati" => &[
        VerbDictionaryEntry { lemma: "prisylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prisęgati" => &[
        VerbDictionaryEntry { lemma: "prisęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prisęgnųti" => &[
        VerbDictionaryEntry { lemma: "prisęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisųditi" => &[
        VerbDictionaryEntry { lemma: "prisųditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prisųđati" => &[
        VerbDictionaryEntry { lemma: "prisųđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pritiskati" => &[
        VerbDictionaryEntry { lemma: "pritiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pritisknųti" => &[
        VerbDictionaryEntry { lemma: "pritisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pritvarjati sę" => &[
        VerbDictionaryEntry { lemma: "pritvarjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "pritvoriti sę" => &[
        VerbDictionaryEntry { lemma: "pritvoriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "pritęgati" => &[
        VerbDictionaryEntry { lemma: "pritęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pritęgnųti" => &[
        VerbDictionaryEntry { lemma: "pritęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priučati" => &[
        VerbDictionaryEntry { lemma: "priučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "priučati sę" => &[
        VerbDictionaryEntry { lemma: "priučati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "priučiti" => &[
        VerbDictionaryEntry { lemma: "priučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priučiti sę" => &[
        VerbDictionaryEntry { lemma: "priučiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "privabiti" => &[
        VerbDictionaryEntry { lemma: "privabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privabjati" => &[
        VerbDictionaryEntry { lemma: "privabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privesti" => &[
        VerbDictionaryEntry { lemma: "privesti", addition: "(privede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privezti" => &[
        VerbDictionaryEntry { lemma: "privezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privitati" => &[
        VerbDictionaryEntry { lemma: "privitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privlåčivati" => &[
        VerbDictionaryEntry { lemma: "privlåčivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privlěkati" => &[
        VerbDictionaryEntry { lemma: "privlěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privlěkti" => &[
        VerbDictionaryEntry { lemma: "privlěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privoditi" => &[
        VerbDictionaryEntry { lemma: "privoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privoliti" => &[
        VerbDictionaryEntry { lemma: "privoliti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privoljati" => &[
        VerbDictionaryEntry { lemma: "privoljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privoziti" => &[
        VerbDictionaryEntry { lemma: "privoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privykati" => &[
        VerbDictionaryEntry { lemma: "privykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "privykati sę" => &[
        VerbDictionaryEntry { lemma: "privykati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "privyknųti" => &[
        VerbDictionaryEntry { lemma: "privyknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privyknųti sę" => &[
        VerbDictionaryEntry { lemma: "privyknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "privęzati" => &[
        VerbDictionaryEntry { lemma: "privęzati", addition: "(privęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "privęzyvati" => &[
        VerbDictionaryEntry { lemma: "privęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prizemjati sę" => &[
        VerbDictionaryEntry { lemma: "prizemjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prizemjiti sę" => &[
        VerbDictionaryEntry { lemma: "prizemjiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "priznati" => &[
        VerbDictionaryEntry { lemma: "priznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "priznavati" => &[
        VerbDictionaryEntry { lemma: "priznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prizvati" => &[
        VerbDictionaryEntry { lemma: "prizvati", addition: "(prizȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prizyvati" => &[
        VerbDictionaryEntry { lemma: "prizyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pričiniti" => &[
        VerbDictionaryEntry { lemma: "pričiniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pričinjati" => &[
        VerbDictionaryEntry { lemma: "pričinjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prišiti" => &[
        VerbDictionaryEntry { lemma: "prišiti", addition: "(prišije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "probijati sę" => &[
        VerbDictionaryEntry { lemma: "probijati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "probiti" => &[
        VerbDictionaryEntry { lemma: "probiti", addition: "(probije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "probiti sę" => &[
        VerbDictionaryEntry { lemma: "probiti sę", addition: "(probije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "probivati" => &[
        VerbDictionaryEntry { lemma: "probivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "probovati" => &[
        VerbDictionaryEntry { lemma: "probovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "probuditi sę" => &[
        VerbDictionaryEntry { lemma: "probuditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "procitovati" => &[
        VerbDictionaryEntry { lemma: "procitovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "procvětati" => &[
        VerbDictionaryEntry { lemma: "procvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prodati" => &[
        VerbDictionaryEntry { lemma: "prodati", addition: "(proda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prodavati" => &[
        VerbDictionaryEntry { lemma: "prodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prodirati sę" => &[
        VerbDictionaryEntry { lemma: "prodirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prodiraviti" => &[
        VerbDictionaryEntry { lemma: "prodiraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prodiravjati" => &[
        VerbDictionaryEntry { lemma: "prodiravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prodrěti sę" => &[
        VerbDictionaryEntry { lemma: "prodrěti sę", addition: "(prodere)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "produkovati" => &[
        VerbDictionaryEntry { lemma: "produkovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "produmati" => &[
        VerbDictionaryEntry { lemma: "produmati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prodŕti sę" => &[
        VerbDictionaryEntry { lemma: "prodŕti sę", addition: "(prodere)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prodȯlžati" => &[
        VerbDictionaryEntry { lemma: "prodȯlžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prodȯlžiti" => &[
        VerbDictionaryEntry { lemma: "prodȯlžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "profanovati" => &[
        VerbDictionaryEntry { lemma: "profanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "profesionalizovati" => &[
        VerbDictionaryEntry { lemma: "profesionalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "proganjati" => &[
        VerbDictionaryEntry { lemma: "proganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proglašati" => &[
        VerbDictionaryEntry { lemma: "proglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proglåsiti" => &[
        VerbDictionaryEntry { lemma: "proglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "programovati" => &[
        VerbDictionaryEntry { lemma: "programovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "progȯltati" => &[
        VerbDictionaryEntry { lemma: "progȯltati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "progȯltnųti" => &[
        VerbDictionaryEntry { lemma: "progȯltnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prohlåditi" => &[
        VerbDictionaryEntry { lemma: "prohlåditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prohlåditi sę" => &[
        VerbDictionaryEntry { lemma: "prohlåditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prohoditi" => &[
        VerbDictionaryEntry { lemma: "prohoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prohoditi sę" => &[
        VerbDictionaryEntry { lemma: "prohoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "proigrati" => &[
        VerbDictionaryEntry { lemma: "proigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "proigryvati" => &[
        VerbDictionaryEntry { lemma: "proigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proiztekti" => &[
        VerbDictionaryEntry { lemma: "proiztekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "proiztěkati" => &[
        VerbDictionaryEntry { lemma: "proiztěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "proizvesti" => &[
        VerbDictionaryEntry { lemma: "proizvesti", addition: "(proizvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "proizvoditi" => &[
        VerbDictionaryEntry { lemma: "proizvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "projaviti" => &[
        VerbDictionaryEntry { lemma: "projaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "projavjati" => &[
        VerbDictionaryEntry { lemma: "projavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "projdti" => &[
        VerbDictionaryEntry { lemma: "projdti", addition: "(projde; prošėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "projehati" => &[
        VerbDictionaryEntry { lemma: "projehati", addition: "(projede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "projektovati" => &[
        VerbDictionaryEntry { lemma: "projektovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proježđati" => &[
        VerbDictionaryEntry { lemma: "proježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "projmati" => &[
        VerbDictionaryEntry { lemma: "projmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "projęti" => &[
        VerbDictionaryEntry { lemma: "projęti", addition: "(projme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "proklęti" => &[
        VerbDictionaryEntry { lemma: "proklęti", addition: "(proklne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "proklęti", addition: "(proklne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prokontrolovati" => &[
        VerbDictionaryEntry { lemma: "prokontrolovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prokrijumčariti" => &[
        VerbDictionaryEntry { lemma: "prokrijumčariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prolamyvati" => &[
        VerbDictionaryEntry { lemma: "prolamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proliti" => &[
        VerbDictionaryEntry { lemma: "proliti", addition: "(prolije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prolivati" => &[
        VerbDictionaryEntry { lemma: "prolivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prolomiti" => &[
        VerbDictionaryEntry { lemma: "prolomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prolězati" => &[
        VerbDictionaryEntry { lemma: "prolězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prolězti" => &[
        VerbDictionaryEntry { lemma: "prolězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pronevěriti" => &[
        VerbDictionaryEntry { lemma: "pronevěriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pronevěrjati" => &[
        VerbDictionaryEntry { lemma: "pronevěrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pronikati" => &[
        VerbDictionaryEntry { lemma: "pronikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "proniknųti" => &[
        VerbDictionaryEntry { lemma: "proniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "propadati" => &[
        VerbDictionaryEntry { lemma: "propadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "propagovati" => &[
        VerbDictionaryEntry { lemma: "propagovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "propasti" => &[
        VerbDictionaryEntry { lemma: "propasti", addition: "(propade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "propiti" => &[
        VerbDictionaryEntry { lemma: "propiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "propivati" => &[
        VerbDictionaryEntry { lemma: "propivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proponovati" => &[
        VerbDictionaryEntry { lemma: "proponovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "propustiti" => &[
        VerbDictionaryEntry { lemma: "propustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "propušćati" => &[
        VerbDictionaryEntry { lemma: "propušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proråstati" => &[
        VerbDictionaryEntry { lemma: "proråstati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prositi" => &[
        VerbDictionaryEntry { lemma: "prositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proslaviti" => &[
        VerbDictionaryEntry { lemma: "proslaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "proslaviti sę" => &[
        VerbDictionaryEntry { lemma: "proslaviti sę", addition: "(+5)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: Some(5) },
    ],
    "proslavjati" => &[
        VerbDictionaryEntry { lemma: "proslavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "proslavjati sę" => &[
        VerbDictionaryEntry { lemma: "proslavjati sę", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(5) },
    ],
    "prospati" => &[
        VerbDictionaryEntry { lemma: "prospati", addition: "(prospi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "prospati", addition: "(prospi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prostirati" => &[
        VerbDictionaryEntry { lemma: "prostirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prostiti" => &[
        VerbDictionaryEntry { lemma: "prostiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prostiti sę" => &[
        VerbDictionaryEntry { lemma: "prostiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prostrěti" => &[
        VerbDictionaryEntry { lemma: "prostrěti", addition: "(prostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prostuditi sę" => &[
        VerbDictionaryEntry { lemma: "prostuditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prostuđati sę" => &[
        VerbDictionaryEntry { lemma: "prostuđati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prostŕti" => &[
        VerbDictionaryEntry { lemma: "prostŕti", addition: "(prostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prosvětiti" => &[
        VerbDictionaryEntry { lemma: "prosvětiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prosvěćati" => &[
        VerbDictionaryEntry { lemma: "prosvěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "protekti" => &[
        VerbDictionaryEntry { lemma: "protekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "protestovati" => &[
        VerbDictionaryEntry { lemma: "protestovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "protivdějati" => &[
        VerbDictionaryEntry { lemma: "protivdějati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "protiviti sę" => &[
        VerbDictionaryEntry { lemma: "protiviti sę", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(3) },
    ],
    "protivrěčiti" => &[
        VerbDictionaryEntry { lemma: "protivrěčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "protrěti" => &[
        VerbDictionaryEntry { lemma: "protrěti", addition: "(protre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "protęgati sę" => &[
        VerbDictionaryEntry { lemma: "protęgati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "protęgnųti sę" => &[
        VerbDictionaryEntry { lemma: "protęgnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "protěkati" => &[
        VerbDictionaryEntry { lemma: "protěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "protŕti" => &[
        VerbDictionaryEntry { lemma: "protŕti", addition: "(protre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "provesti" => &[
        VerbDictionaryEntry { lemma: "provesti", addition: "(provede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "provoditi" => &[
        VerbDictionaryEntry { lemma: "provoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "provoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "provokovati" => &[
        VerbDictionaryEntry { lemma: "provokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "provođati" => &[
        VerbDictionaryEntry { lemma: "provođati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prověriti" => &[
        VerbDictionaryEntry { lemma: "prověriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prověrjati" => &[
        VerbDictionaryEntry { lemma: "prověrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "provětriti" => &[
        VerbDictionaryEntry { lemma: "provětriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "provětrjati" => &[
        VerbDictionaryEntry { lemma: "provětrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "provŕgati" => &[
        VerbDictionaryEntry { lemma: "provŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "provŕgnųti" => &[
        VerbDictionaryEntry { lemma: "provŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "provŕtiti" => &[
        VerbDictionaryEntry { lemma: "provŕtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "provŕćati" => &[
        VerbDictionaryEntry { lemma: "provŕćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pročistiti" => &[
        VerbDictionaryEntry { lemma: "pročistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pročitati" => &[
        VerbDictionaryEntry { lemma: "pročitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pročišćati" => &[
        VerbDictionaryEntry { lemma: "pročišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prošćati" => &[
        VerbDictionaryEntry { lemma: "prošćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prošćati sę" => &[
        VerbDictionaryEntry { lemma: "prošćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "proživati" => &[
        VerbDictionaryEntry { lemma: "proživati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prskati" => &[
        VerbDictionaryEntry { lemma: "prskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prsknųti" => &[
        VerbDictionaryEntry { lemma: "prsknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pråti" => &[
        VerbDictionaryEntry { lemma: "pråti", addition: "(poŕe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pråzdnovati" => &[
        VerbDictionaryEntry { lemma: "pråzdnovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pråšiti" => &[
        VerbDictionaryEntry { lemma: "pråšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pręsti" => &[
        VerbDictionaryEntry { lemma: "pręsti", addition: "(pręde)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "pręsti", addition: "(pręde)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěadresovati" => &[
        VerbDictionaryEntry { lemma: "prěadresovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěbudovati" => &[
        VerbDictionaryEntry { lemma: "prěbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěbudovyvati" => &[
        VerbDictionaryEntry { lemma: "prěbudovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěbyvati" => &[
        VerbDictionaryEntry { lemma: "prěbyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěcěniti" => &[
        VerbDictionaryEntry { lemma: "prěcěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěcěnjati" => &[
        VerbDictionaryEntry { lemma: "prěcěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědati" => &[
        VerbDictionaryEntry { lemma: "prědati", addition: "(prěda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědavati" => &[
        VerbDictionaryEntry { lemma: "prědavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědhoditi" => &[
        VerbDictionaryEntry { lemma: "prědhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědkladati" => &[
        VerbDictionaryEntry { lemma: "prědkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědlagati" => &[
        VerbDictionaryEntry { lemma: "prědlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědložiti" => &[
        VerbDictionaryEntry { lemma: "prědložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpisati" => &[
        VerbDictionaryEntry { lemma: "prědpisati", addition: "(prědpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpisyvati" => &[
        VerbDictionaryEntry { lemma: "prědpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědplatiti" => &[
        VerbDictionaryEntry { lemma: "prědplatiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prědplaćati" => &[
        VerbDictionaryEntry { lemma: "prědplaćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prědpokladati" => &[
        VerbDictionaryEntry { lemma: "prědpokladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpolagati" => &[
        VerbDictionaryEntry { lemma: "prědpolagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpoložiti" => &[
        VerbDictionaryEntry { lemma: "prědpoložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpovědati" => &[
        VerbDictionaryEntry { lemma: "prědpovědati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpověděti" => &[
        VerbDictionaryEntry { lemma: "prědpověděti", addition: "(prědpově)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědpočitati" => &[
        VerbDictionaryEntry { lemma: "prědpočitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědprijmati" => &[
        VerbDictionaryEntry { lemma: "prědprijmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědprijęti" => &[
        VerbDictionaryEntry { lemma: "prědprijęti", addition: "(prědprijme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědskazati" => &[
        VerbDictionaryEntry { lemma: "prědskazati", addition: "(prědskaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědskazyvati" => &[
        VerbDictionaryEntry { lemma: "prědskazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědstati" => &[
        VerbDictionaryEntry { lemma: "prědstati", addition: "(prědstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prědstavati" => &[
        VerbDictionaryEntry { lemma: "prědstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prědstaviti" => &[
        VerbDictionaryEntry { lemma: "prědstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědstavjati" => &[
        VerbDictionaryEntry { lemma: "prědstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědsědati" => &[
        VerbDictionaryEntry { lemma: "prědsědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěduprěditi" => &[
        VerbDictionaryEntry { lemma: "prěduprěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěduprěđati" => &[
        VerbDictionaryEntry { lemma: "prěduprěđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědvidyvati" => &[
        VerbDictionaryEntry { lemma: "prědvidyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědviděti" => &[
        VerbDictionaryEntry { lemma: "prědviděti", addition: "(prědvidi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prědvyšati" => &[
        VerbDictionaryEntry { lemma: "prědvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědčuvati" => &[
        VerbDictionaryEntry { lemma: "prědčuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prědȯjdti" => &[
        VerbDictionaryEntry { lemma: "prědȯjdti", addition: "(prědȯjde; prědšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěględati" => &[
        VerbDictionaryEntry { lemma: "prěględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěględěti" => &[
        VerbDictionaryEntry { lemma: "prěględěti", addition: "(prěględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěhlađati sę" => &[
        VerbDictionaryEntry { lemma: "prěhlađati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěhlåditi sę" => &[
        VerbDictionaryEntry { lemma: "prěhlåditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěhoditi" => &[
        VerbDictionaryEntry { lemma: "prěhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěhytriti" => &[
        VerbDictionaryEntry { lemma: "prěhytriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěhytrjati" => &[
        VerbDictionaryEntry { lemma: "prěhytrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěimenovati" => &[
        VerbDictionaryEntry { lemma: "prěimenovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěimenovyvati" => &[
        VerbDictionaryEntry { lemma: "prěimenovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěinačati" => &[
        VerbDictionaryEntry { lemma: "prěinačati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěinačiti" => &[
        VerbDictionaryEntry { lemma: "prěinačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prějdti" => &[
        VerbDictionaryEntry { lemma: "prějdti", addition: "(prějde; prěšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prějmati" => &[
        VerbDictionaryEntry { lemma: "prějmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prějęti" => &[
        VerbDictionaryEntry { lemma: "prějęti", addition: "(prějme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěkladati" => &[
        VerbDictionaryEntry { lemma: "prěkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěključati" => &[
        VerbDictionaryEntry { lemma: "prěključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěključiti" => &[
        VerbDictionaryEntry { lemma: "prěključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěkryvati sę" => &[
        VerbDictionaryEntry { lemma: "prěkryvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěložiti" => &[
        VerbDictionaryEntry { lemma: "prěložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prělězati" => &[
        VerbDictionaryEntry { lemma: "prělězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prělězti" => &[
        VerbDictionaryEntry { lemma: "prělězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěmagati" => &[
        VerbDictionaryEntry { lemma: "prěmagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěmeblovati" => &[
        VerbDictionaryEntry { lemma: "prěmeblovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěmogti" => &[
        VerbDictionaryEntry { lemma: "prěmogti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěmotati" => &[
        VerbDictionaryEntry { lemma: "prěmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěmotyvati" => &[
        VerbDictionaryEntry { lemma: "prěmotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěměniti" => &[
        VerbDictionaryEntry { lemma: "prěměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěměniti sę" => &[
        VerbDictionaryEntry { lemma: "prěměniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěměnjati" => &[
        VerbDictionaryEntry { lemma: "prěměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěměnjati sę" => &[
        VerbDictionaryEntry { lemma: "prěměnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěměstiti" => &[
        VerbDictionaryEntry { lemma: "prěměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěměstiti sę" => &[
        VerbDictionaryEntry { lemma: "prěměstiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěměšćati" => &[
        VerbDictionaryEntry { lemma: "prěměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěměšćati sę" => &[
        VerbDictionaryEntry { lemma: "prěměšćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěmȯlviti" => &[
        VerbDictionaryEntry { lemma: "prěmȯlviti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěmȯlvjati" => &[
        VerbDictionaryEntry { lemma: "prěmȯlvjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěnapinati" => &[
        VerbDictionaryEntry { lemma: "prěnapinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnapęti" => &[
        VerbDictionaryEntry { lemma: "prěnapęti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnebrěgati" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnebrěgti" => &[
        VerbDictionaryEntry { lemma: "prěnebrěgti", addition: "(prěnebrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnesti" => &[
        VerbDictionaryEntry { lemma: "prěnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnositi" => &[
        VerbDictionaryEntry { lemma: "prěnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěnoćevati" => &[
        VerbDictionaryEntry { lemma: "prěnoćevati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěobraziti" => &[
        VerbDictionaryEntry { lemma: "prěobraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěobraćati" => &[
        VerbDictionaryEntry { lemma: "prěobraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěobražati" => &[
        VerbDictionaryEntry { lemma: "prěobražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěobråtiti" => &[
        VerbDictionaryEntry { lemma: "prěobråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěocěniti" => &[
        VerbDictionaryEntry { lemma: "prěocěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěocěnjati" => &[
        VerbDictionaryEntry { lemma: "prěocěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěoděti sę" => &[
        VerbDictionaryEntry { lemma: "prěoděti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěoděvati sę" => &[
        VerbDictionaryEntry { lemma: "prěoděvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěpirati sę" => &[
        VerbDictionaryEntry { lemma: "prěpirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěpisati" => &[
        VerbDictionaryEntry { lemma: "prěpisati", addition: "(prěpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěpisyvati" => &[
        VerbDictionaryEntry { lemma: "prěpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěpluti" => &[
        VerbDictionaryEntry { lemma: "prěpluti", addition: "(prěplove)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěplyvati" => &[
        VerbDictionaryEntry { lemma: "prěplyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěplyvti" => &[
        VerbDictionaryEntry { lemma: "prěplyvti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěporųčati" => &[
        VerbDictionaryEntry { lemma: "prěporųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěporųčiti" => &[
        VerbDictionaryEntry { lemma: "prěporųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěpraviti" => &[
        VerbDictionaryEntry { lemma: "prěpraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěpravjati" => &[
        VerbDictionaryEntry { lemma: "prěpravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěprogramovati" => &[
        VerbDictionaryEntry { lemma: "prěprogramovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěrastati" => &[
        VerbDictionaryEntry { lemma: "prěrastati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěroditi sę" => &[
        VerbDictionaryEntry { lemma: "prěroditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěrvati" => &[
        VerbDictionaryEntry { lemma: "prěrvati", addition: "(prěrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěryvati" => &[
        VerbDictionaryEntry { lemma: "prěryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěråsti" => &[
        VerbDictionaryEntry { lemma: "prěråsti", addition: "(prěråste)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěråzkazati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazati", addition: "(prěskaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěråzkazyvati" => &[
        VerbDictionaryEntry { lemma: "prěråzkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěrěkati sę" => &[
        VerbDictionaryEntry { lemma: "prěrěkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěrěkti sę" => &[
        VerbDictionaryEntry { lemma: "prěrěkti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěrězati" => &[
        VerbDictionaryEntry { lemma: "prěrězati", addition: "(prěrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěseliti" => &[
        VerbDictionaryEntry { lemma: "prěseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěseliti sę" => &[
        VerbDictionaryEntry { lemma: "prěseliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěseljati" => &[
        VerbDictionaryEntry { lemma: "prěseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěseljati sę" => &[
        VerbDictionaryEntry { lemma: "prěseljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěslušati" => &[
        VerbDictionaryEntry { lemma: "prěslušati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěslušivati" => &[
        VerbDictionaryEntry { lemma: "prěslušivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěslědovati" => &[
        VerbDictionaryEntry { lemma: "prěslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěslěpiti" => &[
        VerbDictionaryEntry { lemma: "prěslěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsměriti" => &[
        VerbDictionaryEntry { lemma: "prěsměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsměrjati" => &[
        VerbDictionaryEntry { lemma: "prěsměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěspati" => &[
        VerbDictionaryEntry { lemma: "prěspati", addition: "(prěspi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěstarati sę" => &[
        VerbDictionaryEntry { lemma: "prěstarati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěstati" => &[
        VerbDictionaryEntry { lemma: "prěstati", addition: "(prěstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěstavati" => &[
        VerbDictionaryEntry { lemma: "prěstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěstaviti" => &[
        VerbDictionaryEntry { lemma: "prěstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstaviti sę" => &[
        VerbDictionaryEntry { lemma: "prěstaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "prěstavjati" => &[
        VerbDictionaryEntry { lemma: "prěstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstavjati sę" => &[
        VerbDictionaryEntry { lemma: "prěstavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "prěstigati" => &[
        VerbDictionaryEntry { lemma: "prěstigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstignųti" => &[
        VerbDictionaryEntry { lemma: "prěstignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstrašiti" => &[
        VerbDictionaryEntry { lemma: "prěstrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstųpati" => &[
        VerbDictionaryEntry { lemma: "prěstųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěstųpiti" => &[
        VerbDictionaryEntry { lemma: "prěstųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsunųti" => &[
        VerbDictionaryEntry { lemma: "prěsunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsuvati" => &[
        VerbDictionaryEntry { lemma: "prěsuvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsvęzati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzati", addition: "(prěsvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsvęzyvati" => &[
        VerbDictionaryEntry { lemma: "prěsvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsědati" => &[
        VerbDictionaryEntry { lemma: "prěsědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěsěkati" => &[
        VerbDictionaryEntry { lemma: "prěsěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsěkti" => &[
        VerbDictionaryEntry { lemma: "prěsěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěsěsti" => &[
        VerbDictionaryEntry { lemma: "prěsěsti", addition: "(prěsěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prětekti" => &[
        VerbDictionaryEntry { lemma: "prětekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěti" => &[
        VerbDictionaryEntry { lemma: "prěti", addition: "(pre)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prětraviti" => &[
        VerbDictionaryEntry { lemma: "prětraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prětravjati" => &[
        VerbDictionaryEntry { lemma: "prětravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prětvarjati" => &[
        VerbDictionaryEntry { lemma: "prětvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prětvoriti" => &[
        VerbDictionaryEntry { lemma: "prětvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prětěkati" => &[
        VerbDictionaryEntry { lemma: "prětěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prětŕpěti" => &[
        VerbDictionaryEntry { lemma: "prětŕpěti", addition: "(prětŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prětȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "prětȯlmačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěuveličati" => &[
        VerbDictionaryEntry { lemma: "prěuveličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěuveličiti" => &[
        VerbDictionaryEntry { lemma: "prěuveličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvariti" => &[
        VerbDictionaryEntry { lemma: "prěvariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvarjati" => &[
        VerbDictionaryEntry { lemma: "prěvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvažati" => &[
        VerbDictionaryEntry { lemma: "prěvažati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "prěvažiti" => &[
        VerbDictionaryEntry { lemma: "prěvažiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "prěvesti" => &[
        VerbDictionaryEntry { lemma: "prěvesti", addition: "(prěvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvezti" => &[
        VerbDictionaryEntry { lemma: "prěvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvladnųti" => &[
        VerbDictionaryEntry { lemma: "prěvladnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvladyvati" => &[
        VerbDictionaryEntry { lemma: "prěvladyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvoditi" => &[
        VerbDictionaryEntry { lemma: "prěvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvoziti" => &[
        VerbDictionaryEntry { lemma: "prěvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvysiti" => &[
        VerbDictionaryEntry { lemma: "prěvysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvyšati" => &[
        VerbDictionaryEntry { lemma: "prěvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "prěvȯzhoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěvȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "prěvȯzȯjdti", addition: "(prěvȯzȯjde; prěvȯzšėl)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prězirati" => &[
        VerbDictionaryEntry { lemma: "prězirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěznačiti" => &[
        VerbDictionaryEntry { lemma: "prěznačiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prězrěti" => &[
        VerbDictionaryEntry { lemma: "prězrěti", addition: "(prězri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prězvati" => &[
        VerbDictionaryEntry { lemma: "prězvati", addition: "(prězȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prězyvati" => &[
        VerbDictionaryEntry { lemma: "prězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěčiti" => &[
        VerbDictionaryEntry { lemma: "prěčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěžiti" => &[
        VerbDictionaryEntry { lemma: "prěžiti", addition: "(prěžive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prěživati" => &[
        VerbDictionaryEntry { lemma: "prěživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "prěžuvati" => &[
        VerbDictionaryEntry { lemma: "prěžuvati", addition: "(prěžuje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "prųžiti" => &[
        VerbDictionaryEntry { lemma: "prųžiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "psihovati" => &[
        VerbDictionaryEntry { lemma: "psihovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "publikovati" => &[
        VerbDictionaryEntry { lemma: "publikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "puhnųti" => &[
        VerbDictionaryEntry { lemma: "puhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pustiti" => &[
        VerbDictionaryEntry { lemma: "pustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "pustošiti" => &[
        VerbDictionaryEntry { lemma: "pustošiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pustěti" => &[
        VerbDictionaryEntry { lemma: "pustěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pušiti" => &[
        VerbDictionaryEntry { lemma: "pušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pušćati" => &[
        VerbDictionaryEntry { lemma: "pušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pyliti" => &[
        VerbDictionaryEntry { lemma: "pyliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pytati" => &[
        VerbDictionaryEntry { lemma: "pytati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pěvati" => &[
        VerbDictionaryEntry { lemma: "pěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pŕděti" => &[
        VerbDictionaryEntry { lemma: "pŕděti", addition: "(pŕdi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pŕti" => &[
        VerbDictionaryEntry { lemma: "pŕti", addition: "(pre)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pųditi" => &[
        VerbDictionaryEntry { lemma: "pųditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pųkati" => &[
        VerbDictionaryEntry { lemma: "pųkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pųknųti" => &[
        VerbDictionaryEntry { lemma: "pųknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "pųtovati" => &[
        VerbDictionaryEntry { lemma: "pųtovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pȯlniti" => &[
        VerbDictionaryEntry { lemma: "pȯlniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "pȯlzati" => &[
        VerbDictionaryEntry { lemma: "pȯlzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "pȯlzti" => &[
        VerbDictionaryEntry { lemma: "pȯlzti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "racionalizovati" => &[
        VerbDictionaryEntry { lemma: "racionalizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "raditi" => &[
        VerbDictionaryEntry { lemma: "raditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "radovati" => &[
        VerbDictionaryEntry { lemma: "radovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "radovati sę" => &[
        VerbDictionaryEntry { lemma: "radovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "raniti" => &[
        VerbDictionaryEntry { lemma: "raniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ratifikovati" => &[
        VerbDictionaryEntry { lemma: "ratifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "raziti" => &[
        VerbDictionaryEntry { lemma: "raziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "reagovati" => &[
        VerbDictionaryEntry { lemma: "reagovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "realizovati" => &[
        VerbDictionaryEntry { lemma: "realizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "recitovati" => &[
        VerbDictionaryEntry { lemma: "recitovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "redagovati" => &[
        VerbDictionaryEntry { lemma: "redagovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "redaktovati" => &[
        VerbDictionaryEntry { lemma: "redaktovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "redukovati" => &[
        VerbDictionaryEntry { lemma: "redukovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "reformovati" => &[
        VerbDictionaryEntry { lemma: "reformovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "regenerovati" => &[
        VerbDictionaryEntry { lemma: "regenerovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "registrovati" => &[
        VerbDictionaryEntry { lemma: "registrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "regulovati" => &[
        VerbDictionaryEntry { lemma: "regulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "reklamovati" => &[
        VerbDictionaryEntry { lemma: "reklamovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rekomendovati" => &[
        VerbDictionaryEntry { lemma: "rekomendovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "rekonstruovati" => &[
        VerbDictionaryEntry { lemma: "rekonstruovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "remontovati" => &[
        VerbDictionaryEntry { lemma: "remontovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "reorganizovati" => &[
        VerbDictionaryEntry { lemma: "reorganizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "reprezentovati" => &[
        VerbDictionaryEntry { lemma: "reprezentovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "reprodukovati" => &[
        VerbDictionaryEntry { lemma: "reprodukovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "restavrovati" => &[
        VerbDictionaryEntry { lemma: "restavrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "revidovati" => &[
        VerbDictionaryEntry { lemma: "revidovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "revti" => &[
        VerbDictionaryEntry { lemma: "revti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "rezervovati" => &[
        VerbDictionaryEntry { lemma: "rezervovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rezumovati" => &[
        VerbDictionaryEntry { lemma: "rezumovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rizikovati" => &[
        VerbDictionaryEntry { lemma: "rizikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "roditi" => &[
        VerbDictionaryEntry { lemma: "roditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "roditi sę" => &[
        VerbDictionaryEntry { lemma: "roditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "rojiti sę" => &[
        VerbDictionaryEntry { lemma: "rojiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "rokirovati" => &[
        VerbDictionaryEntry { lemma: "rokirovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "roptati" => &[
        VerbDictionaryEntry { lemma: "roptati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ruinovati" => &[
        VerbDictionaryEntry { lemma: "ruinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ruměněti" => &[
        VerbDictionaryEntry { lemma: "ruměněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "rusifikovati" => &[
        VerbDictionaryEntry { lemma: "rusifikovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "rušiti" => &[
        VerbDictionaryEntry { lemma: "rušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rvati" => &[
        VerbDictionaryEntry { lemma: "rvati", addition: "(rve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rvati sę" => &[
        VerbDictionaryEntry { lemma: "rvati sę", addition: "(rve)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "rydati" => &[
        VerbDictionaryEntry { lemma: "rydati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "rygati" => &[
        VerbDictionaryEntry { lemma: "rygati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "rygnųti" => &[
        VerbDictionaryEntry { lemma: "rygnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "rymovati" => &[
        VerbDictionaryEntry { lemma: "rymovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rymovati sę" => &[
        VerbDictionaryEntry { lemma: "rymovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "rysovati" => &[
        VerbDictionaryEntry { lemma: "rysovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ryti" => &[
        VerbDictionaryEntry { lemma: "ryti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råbiti" => &[
        VerbDictionaryEntry { lemma: "råbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råbotati" => &[
        VerbDictionaryEntry { lemma: "råbotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råsti" => &[
        VerbDictionaryEntry { lemma: "råsti", addition: "(råste)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råvniti" => &[
        VerbDictionaryEntry { lemma: "råvniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råvniti sę" => &[
        VerbDictionaryEntry { lemma: "råvniti sę", addition: "(+3)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(3) },
    ],
    "råzbirati" => &[
        VerbDictionaryEntry { lemma: "råzbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzbiti" => &[
        VerbDictionaryEntry { lemma: "råzbiti", addition: "(råzbije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzbivati" => &[
        VerbDictionaryEntry { lemma: "råzbivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzbolěti sę" => &[
        VerbDictionaryEntry { lemma: "råzbolěti sę", addition: "(råzbolěje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzbolěvati sę" => &[
        VerbDictionaryEntry { lemma: "råzbolěvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzbrati" => &[
        VerbDictionaryEntry { lemma: "råzbrati", addition: "(råzbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzbuditi" => &[
        VerbDictionaryEntry { lemma: "råzbuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzběgati sę" => &[
        VerbDictionaryEntry { lemma: "råzběgati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzběgti sę" => &[
        VerbDictionaryEntry { lemma: "råzběgti sę", addition: "(råzběži)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzběsniti" => &[
        VerbDictionaryEntry { lemma: "råzběsniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzběsnjati" => &[
        VerbDictionaryEntry { lemma: "råzběsnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzcvěsti" => &[
        VerbDictionaryEntry { lemma: "råzcvěsti", addition: "(råzcvěte)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råzcvětati" => &[
        VerbDictionaryEntry { lemma: "råzcvětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råzdati" => &[
        VerbDictionaryEntry { lemma: "råzdati", addition: "(råzda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdavati" => &[
        VerbDictionaryEntry { lemma: "råzdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdirati" => &[
        VerbDictionaryEntry { lemma: "råzdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdniti sę" => &[
        VerbDictionaryEntry { lemma: "råzdniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzdražniti" => &[
        VerbDictionaryEntry { lemma: "råzdražniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdražnjati" => &[
        VerbDictionaryEntry { lemma: "råzdražnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdrobiti" => &[
        VerbDictionaryEntry { lemma: "råzdrobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdrobjati" => &[
        VerbDictionaryEntry { lemma: "råzdrobjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdrěti" => &[
        VerbDictionaryEntry { lemma: "råzdrěti", addition: "(råzdere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdvajati" => &[
        VerbDictionaryEntry { lemma: "råzdvajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdvojiti" => &[
        VerbDictionaryEntry { lemma: "råzdvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzděliti" => &[
        VerbDictionaryEntry { lemma: "råzděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzděljati" => &[
        VerbDictionaryEntry { lemma: "råzděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzděti" => &[
        VerbDictionaryEntry { lemma: "råzděti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzděti sę" => &[
        VerbDictionaryEntry { lemma: "råzděti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzděvati" => &[
        VerbDictionaryEntry { lemma: "råzděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzděvati sę" => &[
        VerbDictionaryEntry { lemma: "råzděvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzdŕti" => &[
        VerbDictionaryEntry { lemma: "råzdŕti", addition: "(råzdere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdųti" => &[
        VerbDictionaryEntry { lemma: "råzdųti", addition: "(råzdme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdųvati" => &[
        VerbDictionaryEntry { lemma: "råzdųvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdȯlbati" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzdȯlbti" => &[
        VerbDictionaryEntry { lemma: "råzdȯlbti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgadati" => &[
        VerbDictionaryEntry { lemma: "råzgadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgadyvati" => &[
        VerbDictionaryEntry { lemma: "råzgadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzganjati" => &[
        VerbDictionaryEntry { lemma: "råzganjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgarjati sę" => &[
        VerbDictionaryEntry { lemma: "råzgarjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzgladiti" => &[
        VerbDictionaryEntry { lemma: "råzgladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzglađati" => &[
        VerbDictionaryEntry { lemma: "råzglađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzglašati" => &[
        VerbDictionaryEntry { lemma: "råzglašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzglåsiti" => &[
        VerbDictionaryEntry { lemma: "råzglåsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzględati" => &[
        VerbDictionaryEntry { lemma: "råzględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzględati sę" => &[
        VerbDictionaryEntry { lemma: "råzględati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzględěti" => &[
        VerbDictionaryEntry { lemma: "råzględěti", addition: "(råzględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzględěti sę" => &[
        VerbDictionaryEntry { lemma: "råzględěti sę", addition: "(råzględi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzgnati" => &[
        VerbDictionaryEntry { lemma: "råzgnati", addition: "(råzgone)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgněvati" => &[
        VerbDictionaryEntry { lemma: "råzgněvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgorěti sę" => &[
        VerbDictionaryEntry { lemma: "råzgorěti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzgovarjati" => &[
        VerbDictionaryEntry { lemma: "råzgovarjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råzgovoriti sę" => &[
        VerbDictionaryEntry { lemma: "råzgovoriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzgrabiti" => &[
        VerbDictionaryEntry { lemma: "råzgrabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgrabjati" => &[
        VerbDictionaryEntry { lemma: "råzgrabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgraničati" => &[
        VerbDictionaryEntry { lemma: "råzgraničati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgraničiti" => &[
        VerbDictionaryEntry { lemma: "råzgraničiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgrađati" => &[
        VerbDictionaryEntry { lemma: "råzgrađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgromiti" => &[
        VerbDictionaryEntry { lemma: "råzgromiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgryzati" => &[
        VerbDictionaryEntry { lemma: "råzgryzati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgryzti" => &[
        VerbDictionaryEntry { lemma: "råzgryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgråditi" => &[
        VerbDictionaryEntry { lemma: "råzgråditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgrěti" => &[
        VerbDictionaryEntry { lemma: "råzgrěti", addition: "(råzgrěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzgrěvati" => &[
        VerbDictionaryEntry { lemma: "råzgrěvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzhoditi sę" => &[
        VerbDictionaryEntry { lemma: "råzhoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzigrati" => &[
        VerbDictionaryEntry { lemma: "råzigrati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzigrati sę" => &[
        VerbDictionaryEntry { lemma: "råzigrati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzigryvati" => &[
        VerbDictionaryEntry { lemma: "råzigryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzigryvati sę" => &[
        VerbDictionaryEntry { lemma: "råzigryvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råziskati" => &[
        VerbDictionaryEntry { lemma: "råziskati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råziskyvati" => &[
        VerbDictionaryEntry { lemma: "råziskyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjariti" => &[
        VerbDictionaryEntry { lemma: "råzjariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjarjati" => &[
        VerbDictionaryEntry { lemma: "råzjarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjasniti" => &[
        VerbDictionaryEntry { lemma: "råzjasniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjasnjati" => &[
        VerbDictionaryEntry { lemma: "råzjasnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjebati" => &[
        VerbDictionaryEntry { lemma: "råzjebati", addition: "(råzjebe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjebyvati" => &[
        VerbDictionaryEntry { lemma: "råzjebyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjedati" => &[
        VerbDictionaryEntry { lemma: "råzjedati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjediniti" => &[
        VerbDictionaryEntry { lemma: "råzjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjedinjati" => &[
        VerbDictionaryEntry { lemma: "råzjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzjehati sę" => &[
        VerbDictionaryEntry { lemma: "råzjehati sę", addition: "(råzjede)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzjesti" => &[
        VerbDictionaryEntry { lemma: "råzjesti", addition: "(råzje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzježđati sę" => &[
        VerbDictionaryEntry { lemma: "råzježđati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzkajati sę" => &[
        VerbDictionaryEntry { lemma: "råzkajati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzkalati" => &[
        VerbDictionaryEntry { lemma: "råzkalati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkazati" => &[
        VerbDictionaryEntry { lemma: "råzkazati", addition: "(råzkaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkazyvati" => &[
        VerbDictionaryEntry { lemma: "råzkazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkladati" => &[
        VerbDictionaryEntry { lemma: "råzkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzklejati" => &[
        VerbDictionaryEntry { lemma: "råzklejati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzklejiti" => &[
        VerbDictionaryEntry { lemma: "råzklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzklåti" => &[
        VerbDictionaryEntry { lemma: "råzklåti", addition: "(råzkolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkodovati" => &[
        VerbDictionaryEntry { lemma: "råzkodovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkopati" => &[
        VerbDictionaryEntry { lemma: "råzkopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkopyvati" => &[
        VerbDictionaryEntry { lemma: "råzkopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkovati" => &[
        VerbDictionaryEntry { lemma: "råzkovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkrajati" => &[
        VerbDictionaryEntry { lemma: "råzkrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkričati sę" => &[
        VerbDictionaryEntry { lemma: "råzkričati sę", addition: "(råzkriči)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzkrojiti" => &[
        VerbDictionaryEntry { lemma: "råzkrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkryti" => &[
        VerbDictionaryEntry { lemma: "råzkryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkryvati" => &[
        VerbDictionaryEntry { lemma: "råzkryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkrųtiti" => &[
        VerbDictionaryEntry { lemma: "råzkrųtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkrųtiti sę" => &[
        VerbDictionaryEntry { lemma: "råzkrųtiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzkrųćati" => &[
        VerbDictionaryEntry { lemma: "råzkrųćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkrųćati sę" => &[
        VerbDictionaryEntry { lemma: "råzkrųćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzkrȯšiti" => &[
        VerbDictionaryEntry { lemma: "råzkrȯšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkvartirovati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkvartirovyvati" => &[
        VerbDictionaryEntry { lemma: "råzkvartirovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkydati" => &[
        VerbDictionaryEntry { lemma: "råzkydati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzkydyvati" => &[
        VerbDictionaryEntry { lemma: "råzkydyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzladiti" => &[
        VerbDictionaryEntry { lemma: "råzladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlagati" => &[
        VerbDictionaryEntry { lemma: "råzlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlajati sę" => &[
        VerbDictionaryEntry { lemma: "råzlajati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzlamyvati" => &[
        VerbDictionaryEntry { lemma: "råzlamyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlađati" => &[
        VerbDictionaryEntry { lemma: "råzlađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlegati sę" => &[
        VerbDictionaryEntry { lemma: "råzlegati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzlegti sę" => &[
        VerbDictionaryEntry { lemma: "råzlegti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzletěti sę" => &[
        VerbDictionaryEntry { lemma: "råzletěti sę", addition: "(råzleti)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzlikovati sę" => &[
        VerbDictionaryEntry { lemma: "råzlikovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzliti" => &[
        VerbDictionaryEntry { lemma: "råzliti", addition: "(råzlije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlivati" => &[
        VerbDictionaryEntry { lemma: "råzlivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzličati" => &[
        VerbDictionaryEntry { lemma: "råzličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzličati sę" => &[
        VerbDictionaryEntry { lemma: "råzličati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzličiti" => &[
        VerbDictionaryEntry { lemma: "råzličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlomiti" => &[
        VerbDictionaryEntry { lemma: "råzlomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzložiti" => &[
        VerbDictionaryEntry { lemma: "råzložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlěniti sę" => &[
        VerbDictionaryEntry { lemma: "råzlěniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzlětati sę" => &[
        VerbDictionaryEntry { lemma: "råzlětati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzlųčati" => &[
        VerbDictionaryEntry { lemma: "råzlųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzlųčiti" => &[
        VerbDictionaryEntry { lemma: "råzlųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmazati" => &[
        VerbDictionaryEntry { lemma: "råzmazati", addition: "(råzmaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmazyvati" => &[
        VerbDictionaryEntry { lemma: "råzmazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmesti" => &[
        VerbDictionaryEntry { lemma: "råzmesti", addition: "(råzmete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmetati" => &[
        VerbDictionaryEntry { lemma: "råzmetati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmetyvati" => &[
        VerbDictionaryEntry { lemma: "råzmetyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzminovati sę" => &[
        VerbDictionaryEntry { lemma: "råzminovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzminųti sę" => &[
        VerbDictionaryEntry { lemma: "råzminųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzmlåtiti" => &[
        VerbDictionaryEntry { lemma: "råzmlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmlěti" => &[
        VerbDictionaryEntry { lemma: "råzmlěti", addition: "(råzmelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmnažati" => &[
        VerbDictionaryEntry { lemma: "råzmnažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmnažati sę" => &[
        VerbDictionaryEntry { lemma: "råzmnažati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzmnožiti" => &[
        VerbDictionaryEntry { lemma: "råzmnožiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmnožiti sę" => &[
        VerbDictionaryEntry { lemma: "råzmnožiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzmokati" => &[
        VerbDictionaryEntry { lemma: "råzmokati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råzmoknųti" => &[
        VerbDictionaryEntry { lemma: "råzmoknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råzmontovati" => &[
        VerbDictionaryEntry { lemma: "råzmontovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmotati" => &[
        VerbDictionaryEntry { lemma: "råzmotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmražati" => &[
        VerbDictionaryEntry { lemma: "råzmražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmråziti" => &[
        VerbDictionaryEntry { lemma: "råzmråziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmysliti" => &[
        VerbDictionaryEntry { lemma: "råzmysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmysljati" => &[
        VerbDictionaryEntry { lemma: "råzmysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmyti" => &[
        VerbDictionaryEntry { lemma: "råzmyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmyvati" => &[
        VerbDictionaryEntry { lemma: "råzmyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmękčati" => &[
        VerbDictionaryEntry { lemma: "råzmękčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmękčiti" => &[
        VerbDictionaryEntry { lemma: "råzmękčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmęti" => &[
        VerbDictionaryEntry { lemma: "råzmęti", addition: "(råzmne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměniti" => &[
        VerbDictionaryEntry { lemma: "råzměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměnjati" => &[
        VerbDictionaryEntry { lemma: "råzměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměriti" => &[
        VerbDictionaryEntry { lemma: "råzměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměrjati" => &[
        VerbDictionaryEntry { lemma: "råzměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměstiti" => &[
        VerbDictionaryEntry { lemma: "råzměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzmětati" => &[
        VerbDictionaryEntry { lemma: "råzmětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměšati" => &[
        VerbDictionaryEntry { lemma: "råzměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměšivati" => &[
        VerbDictionaryEntry { lemma: "råzměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzměšćati" => &[
        VerbDictionaryEntry { lemma: "råzměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råznesti" => &[
        VerbDictionaryEntry { lemma: "råznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råznesti sę" => &[
        VerbDictionaryEntry { lemma: "råznesti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzniti sę" => &[
        VerbDictionaryEntry { lemma: "råzniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råznositi" => &[
        VerbDictionaryEntry { lemma: "råznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råznositi sę" => &[
        VerbDictionaryEntry { lemma: "råznositi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzorati" => &[
        VerbDictionaryEntry { lemma: "råzorati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzoriti" => &[
        VerbDictionaryEntry { lemma: "råzoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzorjati" => &[
        VerbDictionaryEntry { lemma: "råzorjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzorųžati" => &[
        VerbDictionaryEntry { lemma: "råzorųžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzorųžiti" => &[
        VerbDictionaryEntry { lemma: "råzorųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzočarovati" => &[
        VerbDictionaryEntry { lemma: "råzočarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzočarovyvati" => &[
        VerbDictionaryEntry { lemma: "råzočarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpadati sę" => &[
        VerbDictionaryEntry { lemma: "råzpadati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzpakovati" => &[
        VerbDictionaryEntry { lemma: "råzpakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpakovyvati" => &[
        VerbDictionaryEntry { lemma: "råzpakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpaliti" => &[
        VerbDictionaryEntry { lemma: "råzpaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpaliti sę" => &[
        VerbDictionaryEntry { lemma: "råzpaliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzpaljati" => &[
        VerbDictionaryEntry { lemma: "råzpaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpaljati sę" => &[
        VerbDictionaryEntry { lemma: "råzpaljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzparjati" => &[
        VerbDictionaryEntry { lemma: "råzparjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpasti sę" => &[
        VerbDictionaryEntry { lemma: "råzpasti sę", addition: "(råzpade)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzpečatati" => &[
        VerbDictionaryEntry { lemma: "råzpečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpečatyvati" => &[
        VerbDictionaryEntry { lemma: "råzpečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpinati" => &[
        VerbDictionaryEntry { lemma: "råzpinati", addition: "(råzpne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzplesti" => &[
        VerbDictionaryEntry { lemma: "råzplesti", addition: "(råzplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpletati" => &[
        VerbDictionaryEntry { lemma: "råzpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzplyvati sę" => &[
        VerbDictionaryEntry { lemma: "råzplyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzplyvti sę" => &[
        VerbDictionaryEntry { lemma: "råzplyvti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzporęditi" => &[
        VerbDictionaryEntry { lemma: "råzporęditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzporęđati" => &[
        VerbDictionaryEntry { lemma: "råzporęđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpoznati" => &[
        VerbDictionaryEntry { lemma: "råzpoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpoznavati" => &[
        VerbDictionaryEntry { lemma: "råzpoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprašati" => &[
        VerbDictionaryEntry { lemma: "råzprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprodati" => &[
        VerbDictionaryEntry { lemma: "råzprodati", addition: "(råzproda)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprodavati" => &[
        VerbDictionaryEntry { lemma: "råzprodavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprostirati" => &[
        VerbDictionaryEntry { lemma: "råzprostirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprostirati sę" => &[
        VerbDictionaryEntry { lemma: "råzprostirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzprostranjati" => &[
        VerbDictionaryEntry { lemma: "råzprostranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprostranjati sę" => &[
        VerbDictionaryEntry { lemma: "råzprostranjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzprostråniti" => &[
        VerbDictionaryEntry { lemma: "råzprostråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprostråniti sę" => &[
        VerbDictionaryEntry { lemma: "råzprostråniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzprostrěti" => &[
        VerbDictionaryEntry { lemma: "råzprostrěti", addition: "(råzprostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzprostŕti" => &[
        VerbDictionaryEntry { lemma: "råzprostŕti", addition: "(råzprostre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpråti" => &[
        VerbDictionaryEntry { lemma: "råzpråti", addition: "(råzpoŕe)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpråšiti" => &[
        VerbDictionaryEntry { lemma: "råzpråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpustiti" => &[
        VerbDictionaryEntry { lemma: "råzpustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpušćati" => &[
        VerbDictionaryEntry { lemma: "råzpušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpytati" => &[
        VerbDictionaryEntry { lemma: "råzpytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpytyvati" => &[
        VerbDictionaryEntry { lemma: "råzpytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpęti" => &[
        VerbDictionaryEntry { lemma: "råzpęti", addition: "(råzpne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzpěniti" => &[
        VerbDictionaryEntry { lemma: "råzpěniti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råzradovati" => &[
        VerbDictionaryEntry { lemma: "råzradovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrastati sę" => &[
        VerbDictionaryEntry { lemma: "råzrastati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzraznjati" => &[
        VerbDictionaryEntry { lemma: "råzraznjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzruměniti sę" => &[
        VerbDictionaryEntry { lemma: "råzruměniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzrušati" => &[
        VerbDictionaryEntry { lemma: "råzrušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrušiti" => &[
        VerbDictionaryEntry { lemma: "råzrušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrvati" => &[
        VerbDictionaryEntry { lemma: "råzrvati", addition: "(råzrve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzryti" => &[
        VerbDictionaryEntry { lemma: "råzryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzryvati" => &[
        VerbDictionaryEntry { lemma: "råzryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzråbotati" => &[
        VerbDictionaryEntry { lemma: "råzråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzråbotyvati" => &[
        VerbDictionaryEntry { lemma: "råzråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzråsti sę" => &[
        VerbDictionaryEntry { lemma: "råzråsti sę", addition: "(råzråste)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzråzniti" => &[
        VerbDictionaryEntry { lemma: "råzråzniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrězati" => &[
        VerbDictionaryEntry { lemma: "råzrězati", addition: "(råzrěže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrěšati" => &[
        VerbDictionaryEntry { lemma: "råzrěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrěšiti" => &[
        VerbDictionaryEntry { lemma: "råzrěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrųbati" => &[
        VerbDictionaryEntry { lemma: "råzrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzrųbyvati" => &[
        VerbDictionaryEntry { lemma: "råzrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsaditi" => &[
        VerbDictionaryEntry { lemma: "råzsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsađati" => &[
        VerbDictionaryEntry { lemma: "råzsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsedlati" => &[
        VerbDictionaryEntry { lemma: "råzsedlati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsedlyvati" => &[
        VerbDictionaryEntry { lemma: "råzsedlyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzslati" => &[
        VerbDictionaryEntry { lemma: "råzslati", addition: "(råzšlje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzslěditi" => &[
        VerbDictionaryEntry { lemma: "råzslěditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzslědovati" => &[
        VerbDictionaryEntry { lemma: "råzslědovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsmatrjati" => &[
        VerbDictionaryEntry { lemma: "råzsmatrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsmotriti" => &[
        VerbDictionaryEntry { lemma: "råzsmotriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsmějati sę" => &[
        VerbDictionaryEntry { lemma: "råzsmějati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzsměšati" => &[
        VerbDictionaryEntry { lemma: "råzsměšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsměšiti" => &[
        VerbDictionaryEntry { lemma: "råzsměšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstati sę" => &[
        VerbDictionaryEntry { lemma: "råzstati sę", addition: "(råzstane)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzstavati sę" => &[
        VerbDictionaryEntry { lemma: "råzstavati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzstaviti" => &[
        VerbDictionaryEntry { lemma: "råzstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstavjati" => &[
        VerbDictionaryEntry { lemma: "råzstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsteliti" => &[
        VerbDictionaryEntry { lemma: "råzsteliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstrajati" => &[
        VerbDictionaryEntry { lemma: "råzstrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstrojiti" => &[
        VerbDictionaryEntry { lemma: "råzstrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstrěliti" => &[
        VerbDictionaryEntry { lemma: "råzstrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzstrěljati" => &[
        VerbDictionaryEntry { lemma: "råzstrěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsvirěpěti" => &[
        VerbDictionaryEntry { lemma: "råzsvirěpěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råzsvětliti sę" => &[
        VerbDictionaryEntry { lemma: "råzsvětliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzsvětljati sę" => &[
        VerbDictionaryEntry { lemma: "råzsvětljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzsylati" => &[
        VerbDictionaryEntry { lemma: "råzsylati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsypati" => &[
        VerbDictionaryEntry { lemma: "råzsypati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsypyvati" => &[
        VerbDictionaryEntry { lemma: "råzsypyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsčitati" => &[
        VerbDictionaryEntry { lemma: "råzsčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsčityvati" => &[
        VerbDictionaryEntry { lemma: "råzsčityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsějati" => &[
        VerbDictionaryEntry { lemma: "råzsějati", addition: "(råzsěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsějivati" => &[
        VerbDictionaryEntry { lemma: "råzsějivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsŕditi" => &[
        VerbDictionaryEntry { lemma: "råzsŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzsųditi" => &[
        VerbDictionaryEntry { lemma: "råzsųditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råzsųđati" => &[
        VerbDictionaryEntry { lemma: "råzsųđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råztajati" => &[
        VerbDictionaryEntry { lemma: "råztajati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztapjati" => &[
        VerbDictionaryEntry { lemma: "råztapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztekti sę" => &[
        VerbDictionaryEntry { lemma: "råztekti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råztirati" => &[
        VerbDictionaryEntry { lemma: "råztirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztopiti" => &[
        VerbDictionaryEntry { lemma: "råztopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztratiti" => &[
        VerbDictionaryEntry { lemma: "råztratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztraćati" => &[
        VerbDictionaryEntry { lemma: "råztraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrgati" => &[
        VerbDictionaryEntry { lemma: "råztrgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrgnųti" => &[
        VerbDictionaryEntry { lemma: "råztrgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrojiti" => &[
        VerbDictionaryEntry { lemma: "råztrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrěskati" => &[
        VerbDictionaryEntry { lemma: "råztrěskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrěsknųti" => &[
        VerbDictionaryEntry { lemma: "råztrěsknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrěti" => &[
        VerbDictionaryEntry { lemma: "råztrěti", addition: "(råztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztrųbiti" => &[
        VerbDictionaryEntry { lemma: "råztrųbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztvarjati" => &[
        VerbDictionaryEntry { lemma: "råztvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztvoriti" => &[
        VerbDictionaryEntry { lemma: "råztvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztęgati" => &[
        VerbDictionaryEntry { lemma: "råztęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztęgati sę" => &[
        VerbDictionaryEntry { lemma: "råztęgati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råztęgnųti" => &[
        VerbDictionaryEntry { lemma: "råztęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztęgnųti sę" => &[
        VerbDictionaryEntry { lemma: "råztęgnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råztěkati sę" => &[
        VerbDictionaryEntry { lemma: "råztěkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råztŕti" => &[
        VerbDictionaryEntry { lemma: "råztŕti", addition: "(råztre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztȯlkati" => &[
        VerbDictionaryEntry { lemma: "råztȯlkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råztȯlkti" => &[
        VerbDictionaryEntry { lemma: "råztȯlkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztȯlstěti" => &[
        VerbDictionaryEntry { lemma: "råztȯlstěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "råztȯptati" => &[
        VerbDictionaryEntry { lemma: "råztȯptati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råztȯptyvati" => &[
        VerbDictionaryEntry { lemma: "råztȯptyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzumovati" => &[
        VerbDictionaryEntry { lemma: "råzumovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "råzuměti" => &[
        VerbDictionaryEntry { lemma: "råzuměti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzuti sę" => &[
        VerbDictionaryEntry { lemma: "råzuti sę", addition: "(råzuje)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzuvati sę" => &[
        VerbDictionaryEntry { lemma: "råzuvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvaliti" => &[
        VerbDictionaryEntry { lemma: "råzvaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvaliti sę" => &[
        VerbDictionaryEntry { lemma: "råzvaliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvaljati" => &[
        VerbDictionaryEntry { lemma: "råzvaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvaljati sę" => &[
        VerbDictionaryEntry { lemma: "råzvaljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvažati" => &[
        VerbDictionaryEntry { lemma: "råzvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvažiti" => &[
        VerbDictionaryEntry { lemma: "råzvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzveseliti" => &[
        VerbDictionaryEntry { lemma: "råzveseliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzveseljati" => &[
        VerbDictionaryEntry { lemma: "råzveseljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvesti" => &[
        VerbDictionaryEntry { lemma: "råzvesti", addition: "(råzvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "råzvesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvesti sę" => &[
        VerbDictionaryEntry { lemma: "råzvesti sę", addition: "(råzvede)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvezti" => &[
        VerbDictionaryEntry { lemma: "råzvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzviti" => &[
        VerbDictionaryEntry { lemma: "råzviti", addition: "(råzvije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzviti sę" => &[
        VerbDictionaryEntry { lemma: "råzviti sę", addition: "(råzvije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvivati" => &[
        VerbDictionaryEntry { lemma: "råzvivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvivati sę" => &[
        VerbDictionaryEntry { lemma: "råzvivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvoditi" => &[
        VerbDictionaryEntry { lemma: "råzvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvoditi sę" => &[
        VerbDictionaryEntry { lemma: "råzvoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzvoziti" => &[
        VerbDictionaryEntry { lemma: "råzvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvęzati" => &[
        VerbDictionaryEntry { lemma: "råzvęzati", addition: "(råzvęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvęzyvati" => &[
        VerbDictionaryEntry { lemma: "råzvęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvědati" => &[
        VerbDictionaryEntry { lemma: "råzvědati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzvědyvati" => &[
        VerbDictionaryEntry { lemma: "råzvědyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzčesati" => &[
        VerbDictionaryEntry { lemma: "råzčesati", addition: "(råzčeše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzčistiti" => &[
        VerbDictionaryEntry { lemma: "råzčistiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzčišćati" => &[
        VerbDictionaryEntry { lemma: "råzčišćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzčuliti" => &[
        VerbDictionaryEntry { lemma: "råzčuliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzčuljati" => &[
        VerbDictionaryEntry { lemma: "råzčuljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzšifrovati" => &[
        VerbDictionaryEntry { lemma: "råzšifrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzširiti" => &[
        VerbDictionaryEntry { lemma: "råzširiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzširiti sę" => &[
        VerbDictionaryEntry { lemma: "råzširiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "råzširjati" => &[
        VerbDictionaryEntry { lemma: "råzširjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzširjati sę" => &[
        VerbDictionaryEntry { lemma: "råzširjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "råzšnurovati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzšnurovyvati" => &[
        VerbDictionaryEntry { lemma: "råzšnurovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzščepiti" => &[
        VerbDictionaryEntry { lemma: "råzščepiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzščepjati" => &[
        VerbDictionaryEntry { lemma: "råzščepjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzžegti" => &[
        VerbDictionaryEntry { lemma: "råzžegti", addition: "(råzžže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzžerati" => &[
        VerbDictionaryEntry { lemma: "råzžerati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzžigati" => &[
        VerbDictionaryEntry { lemma: "råzžigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "råzžrěti" => &[
        VerbDictionaryEntry { lemma: "råzžrěti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzžuvati" => &[
        VerbDictionaryEntry { lemma: "råzžuvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "råzȯjdti sę" => &[
        VerbDictionaryEntry { lemma: "råzȯjdti sę", addition: "(råzȯjde; råzšėl)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "rěkti" => &[
        VerbDictionaryEntry { lemma: "rěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rězati" => &[
        VerbDictionaryEntry { lemma: "rězati", addition: "(rěže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rěšati" => &[
        VerbDictionaryEntry { lemma: "rěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rěšiti" => &[
        VerbDictionaryEntry { lemma: "rěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "rųbati" => &[
        VerbDictionaryEntry { lemma: "rųbati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rųkopleskati" => &[
        VerbDictionaryEntry { lemma: "rųkopleskati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "rųkoplesknųti" => &[
        VerbDictionaryEntry { lemma: "rųkoplesknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "rųkovati" => &[
        VerbDictionaryEntry { lemma: "rųkovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rųkovoditi" => &[
        VerbDictionaryEntry { lemma: "rųkovoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "rųčiti sę" => &[
        VerbDictionaryEntry { lemma: "rųčiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sabotovati" => &[
        VerbDictionaryEntry { lemma: "sabotovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "saditi" => &[
        VerbDictionaryEntry { lemma: "saditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "samoubiti" => &[
        VerbDictionaryEntry { lemma: "samoubiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sbirati" => &[
        VerbDictionaryEntry { lemma: "sbirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sbirati sę" => &[
        VerbDictionaryEntry { lemma: "sbirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sbližati sę" => &[
        VerbDictionaryEntry { lemma: "sbližati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sbližiti sę" => &[
        VerbDictionaryEntry { lemma: "sbližiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sbudovati" => &[
        VerbDictionaryEntry { lemma: "sbudovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sběgati sę" => &[
        VerbDictionaryEntry { lemma: "sběgati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sběgti sę" => &[
        VerbDictionaryEntry { lemma: "sběgti sę", addition: "(sběži)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "scati" => &[
        VerbDictionaryEntry { lemma: "scati", addition: "(sci)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sdavati sę" => &[
        VerbDictionaryEntry { lemma: "sdavati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sdeformovati" => &[
        VerbDictionaryEntry { lemma: "sdeformovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sdirati" => &[
        VerbDictionaryEntry { lemma: "sdirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sdrěmnųti" => &[
        VerbDictionaryEntry { lemma: "sdrěmnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sdrěti" => &[
        VerbDictionaryEntry { lemma: "sdrěti", addition: "(sdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sdyhati" => &[
        VerbDictionaryEntry { lemma: "sdyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sdělati" => &[
        VerbDictionaryEntry { lemma: "sdělati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sdŕti" => &[
        VerbDictionaryEntry { lemma: "sdŕti", addition: "(sdre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sdȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sebesmŕtiti" => &[
        VerbDictionaryEntry { lemma: "sebesmŕtiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sedlati" => &[
        VerbDictionaryEntry { lemma: "sedlati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "seliti sę" => &[
        VerbDictionaryEntry { lemma: "seliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sgnesti" => &[
        VerbDictionaryEntry { lemma: "sgnesti", addition: "(sgnete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sgnųti" => &[
        VerbDictionaryEntry { lemma: "sgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sgrěšiti" => &[
        VerbDictionaryEntry { lemma: "sgrěšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sgybati" => &[
        VerbDictionaryEntry { lemma: "sgybati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "shoditi" => &[
        VerbDictionaryEntry { lemma: "shoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "shoditi sę" => &[
        VerbDictionaryEntry { lemma: "shoditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "shovati" => &[
        VerbDictionaryEntry { lemma: "shovati", addition: "(shovaje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "shranjati" => &[
        VerbDictionaryEntry { lemma: "shranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "shråniti" => &[
        VerbDictionaryEntry { lemma: "shråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "shråniti sę" => &[
        VerbDictionaryEntry { lemma: "shråniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "shudnųti" => &[
        VerbDictionaryEntry { lemma: "shudnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "shvatiti" => &[
        VerbDictionaryEntry { lemma: "shvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "signalizovati" => &[
        VerbDictionaryEntry { lemma: "signalizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sikati" => &[
        VerbDictionaryEntry { lemma: "sikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "siliti sę" => &[
        VerbDictionaryEntry { lemma: "siliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "simbolizovati" => &[
        VerbDictionaryEntry { lemma: "simbolizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "simulovati" => &[
        VerbDictionaryEntry { lemma: "simulovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sinhronizovati" => &[
        VerbDictionaryEntry { lemma: "sinhronizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "siněti" => &[
        VerbDictionaryEntry { lemma: "siněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sirotěti" => &[
        VerbDictionaryEntry { lemma: "sirotěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sivěti" => &[
        VerbDictionaryEntry { lemma: "sivěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sjedati" => &[
        VerbDictionaryEntry { lemma: "sjedati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sjediniti" => &[
        VerbDictionaryEntry { lemma: "sjediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sjediniti sę" => &[
        VerbDictionaryEntry { lemma: "sjediniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sjedinjati" => &[
        VerbDictionaryEntry { lemma: "sjedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sjedinjati sę" => &[
        VerbDictionaryEntry { lemma: "sjedinjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sjesti" => &[
        VerbDictionaryEntry { lemma: "sjesti", addition: "(sje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sjęti" => &[
        VerbDictionaryEntry { lemma: "sjęti", addition: "(sȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skakati" => &[
        VerbDictionaryEntry { lemma: "skakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "skanovati" => &[
        VerbDictionaryEntry { lemma: "skanovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "skaržiti sę" => &[
        VerbDictionaryEntry { lemma: "skaržiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "skazati" => &[
        VerbDictionaryEntry { lemma: "skazati", addition: "(skaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skladati" => &[
        VerbDictionaryEntry { lemma: "skladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "skladati sę" => &[
        VerbDictionaryEntry { lemma: "skladati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "skladovati" => &[
        VerbDictionaryEntry { lemma: "skladovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sklanjati" => &[
        VerbDictionaryEntry { lemma: "sklanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sklejiti" => &[
        VerbDictionaryEntry { lemma: "sklejiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skloniti" => &[
        VerbDictionaryEntry { lemma: "skloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skombinovati" => &[
        VerbDictionaryEntry { lemma: "skombinovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skompensovati" => &[
        VerbDictionaryEntry { lemma: "skompensovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skomplikovati" => &[
        VerbDictionaryEntry { lemma: "skomplikovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skonfiskovati" => &[
        VerbDictionaryEntry { lemma: "skonfiskovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skončiti" => &[
        VerbDictionaryEntry { lemma: "skončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skončiti sę" => &[
        VerbDictionaryEntry { lemma: "skončiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "skopiti" => &[
        VerbDictionaryEntry { lemma: "skopiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "skočiti" => &[
        VerbDictionaryEntry { lemma: "skočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "skraćati" => &[
        VerbDictionaryEntry { lemma: "skraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "skrběti" => &[
        VerbDictionaryEntry { lemma: "skrběti", addition: "(skrbi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "skripnųti" => &[
        VerbDictionaryEntry { lemma: "skripnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "skripěti" => &[
        VerbDictionaryEntry { lemma: "skripěti", addition: "(skripi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "skriviti" => &[
        VerbDictionaryEntry { lemma: "skriviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skryti" => &[
        VerbDictionaryEntry { lemma: "skryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skryti sę" => &[
        VerbDictionaryEntry { lemma: "skryti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "skryvati" => &[
        VerbDictionaryEntry { lemma: "skryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "skryvati sę" => &[
        VerbDictionaryEntry { lemma: "skryvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "skråtiti" => &[
        VerbDictionaryEntry { lemma: "skråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skubnųti" => &[
        VerbDictionaryEntry { lemma: "skubnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "skubti" => &[
        VerbDictionaryEntry { lemma: "skubti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "skuhati" => &[
        VerbDictionaryEntry { lemma: "skuhati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "skvrčati" => &[
        VerbDictionaryEntry { lemma: "skvrčati", addition: "(skvrči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "skųpiti sę" => &[
        VerbDictionaryEntry { lemma: "skųpiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "slaběti" => &[
        VerbDictionaryEntry { lemma: "slaběti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slati" => &[
        VerbDictionaryEntry { lemma: "slati", addition: "(šlje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slaviti" => &[
        VerbDictionaryEntry { lemma: "slaviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slaviti sę" => &[
        VerbDictionaryEntry { lemma: "slaviti sę", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: Some(5) },
    ],
    "slgati" => &[
        VerbDictionaryEntry { lemma: "slgati", addition: "(slže)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sliti" => &[
        VerbDictionaryEntry { lemma: "sliti", addition: "(slije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sliti sę" => &[
        VerbDictionaryEntry { lemma: "sliti sę", addition: "(slije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "slivati" => &[
        VerbDictionaryEntry { lemma: "slivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slivati sę" => &[
        VerbDictionaryEntry { lemma: "slivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "slizgati" => &[
        VerbDictionaryEntry { lemma: "slizgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slizgnųti" => &[
        VerbDictionaryEntry { lemma: "slizgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "slomiti" => &[
        VerbDictionaryEntry { lemma: "slomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "slovjanizovati" => &[
        VerbDictionaryEntry { lemma: "slovjanizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "složiti" => &[
        VerbDictionaryEntry { lemma: "složiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "složiti sę" => &[
        VerbDictionaryEntry { lemma: "složiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "slučati sę" => &[
        VerbDictionaryEntry { lemma: "slučati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "slučiti sę" => &[
        VerbDictionaryEntry { lemma: "slučiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "slušati" => &[
        VerbDictionaryEntry { lemma: "slušati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "služiti" => &[
        VerbDictionaryEntry { lemma: "služiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slynųti" => &[
        VerbDictionaryEntry { lemma: "slynųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slyti" => &[
        VerbDictionaryEntry { lemma: "slyti", addition: "(slyne)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slyšati" => &[
        VerbDictionaryEntry { lemma: "slyšati", addition: "(slyši)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slåditi" => &[
        VerbDictionaryEntry { lemma: "slåditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slěditi" => &[
        VerbDictionaryEntry { lemma: "slěditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slěpnųti" => &[
        VerbDictionaryEntry { lemma: "slěpnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "slųčati" => &[
        VerbDictionaryEntry { lemma: "slųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "slųčiti" => &[
        VerbDictionaryEntry { lemma: "slųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smatrjati" => &[
        VerbDictionaryEntry { lemma: "smatrjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "smažiti" => &[
        VerbDictionaryEntry { lemma: "smažiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smlåtiti" => &[
        VerbDictionaryEntry { lemma: "smlåtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smlěti" => &[
        VerbDictionaryEntry { lemma: "smlěti", addition: "(smelje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smogti" => &[
        VerbDictionaryEntry { lemma: "smogti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smoliti" => &[
        VerbDictionaryEntry { lemma: "smoliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smrkati" => &[
        VerbDictionaryEntry { lemma: "smrkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "smrkati sę" => &[
        VerbDictionaryEntry { lemma: "smrkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "smrščiti" => &[
        VerbDictionaryEntry { lemma: "smrščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smrščiti sę" => &[
        VerbDictionaryEntry { lemma: "smrščiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "smućati" => &[
        VerbDictionaryEntry { lemma: "smućati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smykati" => &[
        VerbDictionaryEntry { lemma: "smykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smėnšati" => &[
        VerbDictionaryEntry { lemma: "smėnšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smėnšiti" => &[
        VerbDictionaryEntry { lemma: "smėnšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smękčati" => &[
        VerbDictionaryEntry { lemma: "smękčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smękčiti" => &[
        VerbDictionaryEntry { lemma: "smękčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smęčkati" => &[
        VerbDictionaryEntry { lemma: "smęčkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smějati sę" => &[
        VerbDictionaryEntry { lemma: "smějati sę", addition: "(směje)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "směti" => &[
        VerbDictionaryEntry { lemma: "směti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "směšati" => &[
        VerbDictionaryEntry { lemma: "směšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "směšivati" => &[
        VerbDictionaryEntry { lemma: "směšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "smŕditi" => &[
        VerbDictionaryEntry { lemma: "smŕditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "smųtiti" => &[
        VerbDictionaryEntry { lemma: "smųtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "smųćati" => &[
        VerbDictionaryEntry { lemma: "smųćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snabděti" => &[
        VerbDictionaryEntry { lemma: "snabděti", addition: "(snabdi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "snabděvati" => &[
        VerbDictionaryEntry { lemma: "snabděvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snesti" => &[
        VerbDictionaryEntry { lemma: "snesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "snetvarjati" => &[
        VerbDictionaryEntry { lemma: "snetvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snetvoriti" => &[
        VerbDictionaryEntry { lemma: "snetvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "snimati" => &[
        VerbDictionaryEntry { lemma: "snimati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "snimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sniti" => &[
        VerbDictionaryEntry { lemma: "sniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sniziti" => &[
        VerbDictionaryEntry { lemma: "sniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "snižati" => &[
        VerbDictionaryEntry { lemma: "snižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snositi" => &[
        VerbDictionaryEntry { lemma: "snositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snovati" => &[
        VerbDictionaryEntry { lemma: "snovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "snědati" => &[
        VerbDictionaryEntry { lemma: "snědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sněžiti" => &[
        VerbDictionaryEntry { lemma: "sněžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "soliti" => &[
        VerbDictionaryEntry { lemma: "soliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sondovati" => &[
        VerbDictionaryEntry { lemma: "sondovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sortovati" => &[
        VerbDictionaryEntry { lemma: "sortovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spadati" => &[
        VerbDictionaryEntry { lemma: "spadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spakovati" => &[
        VerbDictionaryEntry { lemma: "spakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spakovati sę" => &[
        VerbDictionaryEntry { lemma: "spakovati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spaliti" => &[
        VerbDictionaryEntry { lemma: "spaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spasati" => &[
        VerbDictionaryEntry { lemma: "spasati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spasti" => &[
        VerbDictionaryEntry { lemma: "spasti", addition: "(spade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "spasti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spati" => &[
        VerbDictionaryEntry { lemma: "spati", addition: "(spi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spekulovati" => &[
        VerbDictionaryEntry { lemma: "spekulovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spirati sę" => &[
        VerbDictionaryEntry { lemma: "spirati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "spisyvati" => &[
        VerbDictionaryEntry { lemma: "spisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "splesti" => &[
        VerbDictionaryEntry { lemma: "splesti", addition: "(splete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sploditi" => &[
        VerbDictionaryEntry { lemma: "sploditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spodobati sę" => &[
        VerbDictionaryEntry { lemma: "spodobati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spolupracovati" => &[
        VerbDictionaryEntry { lemma: "spolupracovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spoluråbotyvati" => &[
        VerbDictionaryEntry { lemma: "spoluråbotyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spomaliti" => &[
        VerbDictionaryEntry { lemma: "spomaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spomaljati" => &[
        VerbDictionaryEntry { lemma: "spomaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spominati" => &[
        VerbDictionaryEntry { lemma: "spominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spomněti" => &[
        VerbDictionaryEntry { lemma: "spomněti", addition: "(spomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sporiti" => &[
        VerbDictionaryEntry { lemma: "sporiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spotiti sę" => &[
        VerbDictionaryEntry { lemma: "spotiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spotknųti sę" => &[
        VerbDictionaryEntry { lemma: "spotknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spotrěbiti" => &[
        VerbDictionaryEntry { lemma: "spotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spotrěbovati" => &[
        VerbDictionaryEntry { lemma: "spotrěbovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spotykati sę" => &[
        VerbDictionaryEntry { lemma: "spotykati sę", addition: "(spotyče)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "spotěti" => &[
        VerbDictionaryEntry { lemma: "spotěti", addition: "(spoti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "spozdniti sę" => &[
        VerbDictionaryEntry { lemma: "spozdniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spozdnjati sę" => &[
        VerbDictionaryEntry { lemma: "spozdnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "spoznati" => &[
        VerbDictionaryEntry { lemma: "spoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spoznavati" => &[
        VerbDictionaryEntry { lemma: "spoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sprašati" => &[
        VerbDictionaryEntry { lemma: "sprašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sprijateliti sę" => &[
        VerbDictionaryEntry { lemma: "sprijateliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sprositi" => &[
        VerbDictionaryEntry { lemma: "sprositi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sprotiviti sę" => &[
        VerbDictionaryEntry { lemma: "sprotiviti sę", addition: "(+3)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: Some(3) },
    ],
    "spręgati" => &[
        VerbDictionaryEntry { lemma: "spręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spręsti" => &[
        VerbDictionaryEntry { lemma: "spręsti", addition: "(spręde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sprěti sę" => &[
        VerbDictionaryEntry { lemma: "sprěti sę", addition: "(spre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spuhnųti" => &[
        VerbDictionaryEntry { lemma: "spuhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "spustiti" => &[
        VerbDictionaryEntry { lemma: "spustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spustiti sę" => &[
        VerbDictionaryEntry { lemma: "spustiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "spušćati" => &[
        VerbDictionaryEntry { lemma: "spušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "spušćati sę" => &[
        VerbDictionaryEntry { lemma: "spušćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "spytati" => &[
        VerbDictionaryEntry { lemma: "spytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spěvati" => &[
        VerbDictionaryEntry { lemma: "spěvati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "spěšiti" => &[
        VerbDictionaryEntry { lemma: "spěšiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "spŕti sę" => &[
        VerbDictionaryEntry { lemma: "spŕti sę", addition: "(spre)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "srastati sę" => &[
        VerbDictionaryEntry { lemma: "srastati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "srati" => &[
        VerbDictionaryEntry { lemma: "srati", addition: "(sere)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sruinovati" => &[
        VerbDictionaryEntry { lemma: "sruinovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sråmiti" => &[
        VerbDictionaryEntry { lemma: "sråmiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sråsti sę" => &[
        VerbDictionaryEntry { lemma: "sråsti sę", addition: "(sråste)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sråvniti" => &[
        VerbDictionaryEntry { lemma: "sråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sråvnjati" => &[
        VerbDictionaryEntry { lemma: "sråvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sråzuměti" => &[
        VerbDictionaryEntry { lemma: "sråzuměti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "srųbati" => &[
        VerbDictionaryEntry { lemma: "srųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "srųbyvati" => &[
        VerbDictionaryEntry { lemma: "srųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ssědati sę" => &[
        VerbDictionaryEntry { lemma: "ssědati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "ssěsti sę" => &[
        VerbDictionaryEntry { lemma: "ssěsti sę", addition: "(ssěde)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "stabilizovati" => &[
        VerbDictionaryEntry { lemma: "stabilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "standardizovati" => &[
        VerbDictionaryEntry { lemma: "standardizovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stanoviti" => &[
        VerbDictionaryEntry { lemma: "stanoviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stanųti" => &[
        VerbDictionaryEntry { lemma: "stanųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "stapjati" => &[
        VerbDictionaryEntry { lemma: "stapjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "starati sę" => &[
        VerbDictionaryEntry { lemma: "starati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "startovati" => &[
        VerbDictionaryEntry { lemma: "startovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "starěti" => &[
        VerbDictionaryEntry { lemma: "starěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "stati" => &[
        VerbDictionaryEntry { lemma: "stati", addition: "(stane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stati sę" => &[
        VerbDictionaryEntry { lemma: "stati sę", addition: "(stane)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "stavati" => &[
        VerbDictionaryEntry { lemma: "stavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "staviti" => &[
        VerbDictionaryEntry { lemma: "staviti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stavjati" => &[
        VerbDictionaryEntry { lemma: "stavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stačiti" => &[
        VerbDictionaryEntry { lemma: "stačiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "stekti" => &[
        VerbDictionaryEntry { lemma: "stekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "steliti" => &[
        VerbDictionaryEntry { lemma: "steliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sterilizovati" => &[
        VerbDictionaryEntry { lemma: "sterilizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stigati" => &[
        VerbDictionaryEntry { lemma: "stigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stignųti" => &[
        VerbDictionaryEntry { lemma: "stignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stimulovati" => &[
        VerbDictionaryEntry { lemma: "stimulovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stiskati" => &[
        VerbDictionaryEntry { lemma: "stiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stisknųti" => &[
        VerbDictionaryEntry { lemma: "stisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stlati" => &[
        VerbDictionaryEntry { lemma: "stlati", addition: "(stelje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stojati" => &[
        VerbDictionaryEntry { lemma: "stojati", addition: "(stoji)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "stojati", addition: "(stoji)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stonati" => &[
        VerbDictionaryEntry { lemma: "stonati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "stopiti" => &[
        VerbDictionaryEntry { lemma: "stopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stradati" => &[
        VerbDictionaryEntry { lemma: "stradati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "strahovati" => &[
        VerbDictionaryEntry { lemma: "strahovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strajkovati" => &[
        VerbDictionaryEntry { lemma: "strajkovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "strašiti" => &[
        VerbDictionaryEntry { lemma: "strašiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strigti" => &[
        VerbDictionaryEntry { lemma: "strigti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strimati" => &[
        VerbDictionaryEntry { lemma: "strimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "strimyvati" => &[
        VerbDictionaryEntry { lemma: "strimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strojiti" => &[
        VerbDictionaryEntry { lemma: "strojiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strugati" => &[
        VerbDictionaryEntry { lemma: "strugati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strčiti" => &[
        VerbDictionaryEntry { lemma: "strčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "strěgti" => &[
        VerbDictionaryEntry { lemma: "strěgti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strěgti sę" => &[
        VerbDictionaryEntry { lemma: "strěgti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "strěliti" => &[
        VerbDictionaryEntry { lemma: "strěliti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "strěljati" => &[
        VerbDictionaryEntry { lemma: "strěljati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "strěsti" => &[
        VerbDictionaryEntry { lemma: "strěsti", addition: "(strěte)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "strěsti sę" => &[
        VerbDictionaryEntry { lemma: "strěsti sę", addition: "(strěte)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "strěćati" => &[
        VerbDictionaryEntry { lemma: "strěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "strěćati sę" => &[
        VerbDictionaryEntry { lemma: "strěćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "studiovati" => &[
        VerbDictionaryEntry { lemma: "studiovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "studiti" => &[
        VerbDictionaryEntry { lemma: "studiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stukati" => &[
        VerbDictionaryEntry { lemma: "stukati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "stuknųti" => &[
        VerbDictionaryEntry { lemma: "stuknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "stvarjati" => &[
        VerbDictionaryEntry { lemma: "stvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "stvoriti" => &[
        VerbDictionaryEntry { lemma: "stvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "stųpati" => &[
        VerbDictionaryEntry { lemma: "stųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "stųpiti" => &[
        VerbDictionaryEntry { lemma: "stųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sunųti" => &[
        VerbDictionaryEntry { lemma: "sunųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "surfovati" => &[
        VerbDictionaryEntry { lemma: "surfovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "suvati" => &[
        VerbDictionaryEntry { lemma: "suvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sušiti" => &[
        VerbDictionaryEntry { lemma: "sušiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svabiti" => &[
        VerbDictionaryEntry { lemma: "svabiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "svariti" => &[
        VerbDictionaryEntry { lemma: "svariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "svarjati" => &[
        VerbDictionaryEntry { lemma: "svarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svatiti sę" => &[
        VerbDictionaryEntry { lemma: "svatiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "svistati" => &[
        VerbDictionaryEntry { lemma: "svistati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "svistnųti" => &[
        VerbDictionaryEntry { lemma: "svistnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sviti" => &[
        VerbDictionaryEntry { lemma: "sviti", addition: "(svije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "svętiti" => &[
        VerbDictionaryEntry { lemma: "svętiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svęzati" => &[
        VerbDictionaryEntry { lemma: "svęzati", addition: "(svęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "svęzyvati" => &[
        VerbDictionaryEntry { lemma: "svęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svědčiti" => &[
        VerbDictionaryEntry { lemma: "svědčiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "svěriti sę" => &[
        VerbDictionaryEntry { lemma: "svěriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "svěrjati sę" => &[
        VerbDictionaryEntry { lemma: "svěrjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "světiti" => &[
        VerbDictionaryEntry { lemma: "světiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "svŕběti" => &[
        VerbDictionaryEntry { lemma: "svŕběti", addition: "(svŕbi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svŕgati" => &[
        VerbDictionaryEntry { lemma: "svŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "svŕgnųti" => &[
        VerbDictionaryEntry { lemma: "svŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sytiti" => &[
        VerbDictionaryEntry { lemma: "sytiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sčisliti" => &[
        VerbDictionaryEntry { lemma: "sčisliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sčitati" => &[
        VerbDictionaryEntry { lemma: "sčitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sęgati" => &[
        VerbDictionaryEntry { lemma: "sęgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sęgnųti" => &[
        VerbDictionaryEntry { lemma: "sęgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sědati" => &[
        VerbDictionaryEntry { lemma: "sědati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sědnųti" => &[
        VerbDictionaryEntry { lemma: "sědnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sěděti" => &[
        VerbDictionaryEntry { lemma: "sěděti", addition: "(sědi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sějati" => &[
        VerbDictionaryEntry { lemma: "sějati", addition: "(sěje)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sěkati" => &[
        VerbDictionaryEntry { lemma: "sěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sěknųti" => &[
        VerbDictionaryEntry { lemma: "sěknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sěkti" => &[
        VerbDictionaryEntry { lemma: "sěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sěsti" => &[
        VerbDictionaryEntry { lemma: "sěsti", addition: "(sěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sŕditi" => &[
        VerbDictionaryEntry { lemma: "sŕditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųditi" => &[
        VerbDictionaryEntry { lemma: "sųditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųglašati sę" => &[
        VerbDictionaryEntry { lemma: "sųglašati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sųglåsiti sę" => &[
        VerbDictionaryEntry { lemma: "sųglåsiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sųharmonizovati" => &[
        VerbDictionaryEntry { lemma: "sųharmonizovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sųmněvati sę" => &[
        VerbDictionaryEntry { lemma: "sųmněvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sųmŕkati sę" => &[
        VerbDictionaryEntry { lemma: "sųmŕkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sųmŕknųti sę" => &[
        VerbDictionaryEntry { lemma: "sųmŕknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sųpostaviti" => &[
        VerbDictionaryEntry { lemma: "sųpostaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sųpostavjati" => &[
        VerbDictionaryEntry { lemma: "sųpostavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųprovađati" => &[
        VerbDictionaryEntry { lemma: "sųprovađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųprovoditi" => &[
        VerbDictionaryEntry { lemma: "sųprovoditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sųråbotati" => &[
        VerbDictionaryEntry { lemma: "sųråbotati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sųsrědotočati" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųsrědotočati sę" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sųsrědotočiti" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sųsrědotočiti sę" => &[
        VerbDictionaryEntry { lemma: "sųsrědotočiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sųstrěsti" => &[
        VerbDictionaryEntry { lemma: "sųstrěsti", addition: "(sųstrěte)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sųstrěćati" => &[
        VerbDictionaryEntry { lemma: "sųstrěćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųt" => &[
        VerbDictionaryEntry { lemma: "sųt", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sųtruditi" => &[
        VerbDictionaryEntry { lemma: "sųtruditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sųtstvovati" => &[
        VerbDictionaryEntry { lemma: "sųtstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯbrati" => &[
        VerbDictionaryEntry { lemma: "sȯbrati", addition: "(sbere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯbrati sę" => &[
        VerbDictionaryEntry { lemma: "sȯbrati sę", addition: "(sbere)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sȯdŕžati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯdŕživati" => &[
        VerbDictionaryEntry { lemma: "sȯdŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯglašati sę" => &[
        VerbDictionaryEntry { lemma: "sȯglašati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sȯglåsiti sę" => &[
        VerbDictionaryEntry { lemma: "sȯglåsiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sȯhnųti" => &[
        VerbDictionaryEntry { lemma: "sȯhnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯjdti" => &[
        VerbDictionaryEntry { lemma: "sȯjdti", addition: "(sȯjde; sȯšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯjdti sę" => &[
        VerbDictionaryEntry { lemma: "sȯjdti sę", addition: "(sȯjde; sȯšėl)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "sȯmknųti" => &[
        VerbDictionaryEntry { lemma: "sȯmknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯobćati" => &[
        VerbDictionaryEntry { lemma: "sȯobćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯobćati sę" => &[
        VerbDictionaryEntry { lemma: "sȯobćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "sȯobćiti" => &[
        VerbDictionaryEntry { lemma: "sȯobćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯsati" => &[
        VerbDictionaryEntry { lemma: "sȯsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯstaviti" => &[
        VerbDictionaryEntry { lemma: "sȯstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯstavjati" => &[
        VerbDictionaryEntry { lemma: "sȯstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯtkati" => &[
        VerbDictionaryEntry { lemma: "sȯtkati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯvladnųti" => &[
        VerbDictionaryEntry { lemma: "sȯvladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯvladyvati" => &[
        VerbDictionaryEntry { lemma: "sȯvladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯvpadati" => &[
        VerbDictionaryEntry { lemma: "sȯvpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯvětovati" => &[
        VerbDictionaryEntry { lemma: "sȯvětovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯvŕšati" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "sȯvŕšati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯvŕšiti" => &[
        VerbDictionaryEntry { lemma: "sȯvŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "sȯvŕšiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯzdati" => &[
        VerbDictionaryEntry { lemma: "sȯzdati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯzdavati" => &[
        VerbDictionaryEntry { lemma: "sȯzdavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯzvati" => &[
        VerbDictionaryEntry { lemma: "sȯzvati", addition: "(sȯzȯve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯzyvati" => &[
        VerbDictionaryEntry { lemma: "sȯzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯčuvstvovati" => &[
        VerbDictionaryEntry { lemma: "sȯčuvstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "sȯžrti" => &[
        VerbDictionaryEntry { lemma: "sȯžrti", addition: "(sȯžre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "sȯžrěti" => &[
        VerbDictionaryEntry { lemma: "sȯžrěti", addition: "(sȯžre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "tajati" => &[
        VerbDictionaryEntry { lemma: "tajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tajiti" => &[
        VerbDictionaryEntry { lemma: "tajiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tancevati" => &[
        VerbDictionaryEntry { lemma: "tancevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tatuovati" => &[
        VerbDictionaryEntry { lemma: "tatuovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "tekti" => &[
        VerbDictionaryEntry { lemma: "tekti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "telefonovati" => &[
        VerbDictionaryEntry { lemma: "telefonovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "teliti sę" => &[
        VerbDictionaryEntry { lemma: "teliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "tesati" => &[
        VerbDictionaryEntry { lemma: "tesati", addition: "(teše)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "testovati" => &[
        VerbDictionaryEntry { lemma: "testovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tipkati" => &[
        VerbDictionaryEntry { lemma: "tipkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tiskati" => &[
        VerbDictionaryEntry { lemma: "tiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tisknųti" => &[
        VerbDictionaryEntry { lemma: "tisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "tkati" => &[
        VerbDictionaryEntry { lemma: "tkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tknųti" => &[
        VerbDictionaryEntry { lemma: "tknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "tknųti sę" => &[
        VerbDictionaryEntry { lemma: "tknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "tlěti" => &[
        VerbDictionaryEntry { lemma: "tlěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tonųti" => &[
        VerbDictionaryEntry { lemma: "tonųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "topiti" => &[
        VerbDictionaryEntry { lemma: "topiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "točiti" => &[
        VerbDictionaryEntry { lemma: "točiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "točiti sę" => &[
        VerbDictionaryEntry { lemma: "točiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "trajati" => &[
        VerbDictionaryEntry { lemma: "trajati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "transkribovati" => &[
        VerbDictionaryEntry { lemma: "transkribovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "transliterovati" => &[
        VerbDictionaryEntry { lemma: "transliterovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "transportovati" => &[
        VerbDictionaryEntry { lemma: "transportovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "tratiti" => &[
        VerbDictionaryEntry { lemma: "tratiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "travmatizovati" => &[
        VerbDictionaryEntry { lemma: "travmatizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "trenovati" => &[
        VerbDictionaryEntry { lemma: "trenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "trepetati" => &[
        VerbDictionaryEntry { lemma: "trepetati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "trgati" => &[
        VerbDictionaryEntry { lemma: "trgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "trgnųti" => &[
        VerbDictionaryEntry { lemma: "trgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "trgovati" => &[
        VerbDictionaryEntry { lemma: "trgovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "trimati" => &[
        VerbDictionaryEntry { lemma: "trimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "truditi sę" => &[
        VerbDictionaryEntry { lemma: "truditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "trvati" => &[
        VerbDictionaryEntry { lemma: "trvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "trėvožiti" => &[
        VerbDictionaryEntry { lemma: "trėvožiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "trėvožiti sę" => &[
        VerbDictionaryEntry { lemma: "trėvožiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "tręsti" => &[
        VerbDictionaryEntry { lemma: "tręsti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tręsti sę" => &[
        VerbDictionaryEntry { lemma: "tręsti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "trěbovati" => &[
        VerbDictionaryEntry { lemma: "trěbovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "trěti" => &[
        VerbDictionaryEntry { lemma: "trěti", addition: "(tre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "trězvěti" => &[
        VerbDictionaryEntry { lemma: "trězvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "trųbiti" => &[
        VerbDictionaryEntry { lemma: "trųbiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tvoriti" => &[
        VerbDictionaryEntry { lemma: "tvoriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tvŕditi" => &[
        VerbDictionaryEntry { lemma: "tvŕditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tvŕdnųti" => &[
        VerbDictionaryEntry { lemma: "tvŕdnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tvŕděti" => &[
        VerbDictionaryEntry { lemma: "tvŕděti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tykati" => &[
        VerbDictionaryEntry { lemma: "tykati", addition: "(tyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "tykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tykati sę" => &[
        VerbDictionaryEntry { lemma: "tykati sę", addition: "(tyče)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "tėmněti" => &[
        VerbDictionaryEntry { lemma: "tėmněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tęgati" => &[
        VerbDictionaryEntry { lemma: "tęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tęgnųti" => &[
        VerbDictionaryEntry { lemma: "tęgnųti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tŕpěti" => &[
        VerbDictionaryEntry { lemma: "tŕpěti", addition: "(tŕpi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "tŕpěti", addition: "(tŕpi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tŕti" => &[
        VerbDictionaryEntry { lemma: "tŕti", addition: "(tre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "tųpěti" => &[
        VerbDictionaryEntry { lemma: "tųpěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tųžiti" => &[
        VerbDictionaryEntry { lemma: "tųžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tųžiti sę" => &[
        VerbDictionaryEntry { lemma: "tųžiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "tȯlkti" => &[
        VerbDictionaryEntry { lemma: "tȯlkti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "tȯlmačiti" => &[
        VerbDictionaryEntry { lemma: "tȯlmačiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ubiti" => &[
        VerbDictionaryEntry { lemma: "ubiti", addition: "(ubije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ubivati" => &[
        VerbDictionaryEntry { lemma: "ubivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uběditi" => &[
        VerbDictionaryEntry { lemma: "uběditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uběditi sę" => &[
        VerbDictionaryEntry { lemma: "uběditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "uběgati" => &[
        VerbDictionaryEntry { lemma: "uběgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "uběgti" => &[
        VerbDictionaryEntry { lemma: "uběgti", addition: "(uběži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uběđati" => &[
        VerbDictionaryEntry { lemma: "uběđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uběđati sę" => &[
        VerbDictionaryEntry { lemma: "uběđati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "udaliti" => &[
        VerbDictionaryEntry { lemma: "udaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udaljati" => &[
        VerbDictionaryEntry { lemma: "udaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "udariti" => &[
        VerbDictionaryEntry { lemma: "udariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udarjati" => &[
        VerbDictionaryEntry { lemma: "udarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "udati sę" => &[
        VerbDictionaryEntry { lemma: "udati sę", addition: "(uda)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "udavati sę" => &[
        VerbDictionaryEntry { lemma: "udavati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "udaviti" => &[
        VerbDictionaryEntry { lemma: "udaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udiviti" => &[
        VerbDictionaryEntry { lemma: "udiviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udivjati" => &[
        VerbDictionaryEntry { lemma: "udivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "udoskonaliti" => &[
        VerbDictionaryEntry { lemma: "udoskonaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udušiti" => &[
        VerbDictionaryEntry { lemma: "udušiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udvojiti" => &[
        VerbDictionaryEntry { lemma: "udvojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uděliti" => &[
        VerbDictionaryEntry { lemma: "uděliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uděljati" => &[
        VerbDictionaryEntry { lemma: "uděljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "udŕžati" => &[
        VerbDictionaryEntry { lemma: "udŕžati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "udŕživati" => &[
        VerbDictionaryEntry { lemma: "udŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ugadati" => &[
        VerbDictionaryEntry { lemma: "ugadati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ugadyvati" => &[
        VerbDictionaryEntry { lemma: "ugadyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ugasati" => &[
        VerbDictionaryEntry { lemma: "ugasati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ugasnųti" => &[
        VerbDictionaryEntry { lemma: "ugasnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uględati" => &[
        VerbDictionaryEntry { lemma: "uględati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uględěti" => &[
        VerbDictionaryEntry { lemma: "uględěti", addition: "(uględi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ugryzti" => &[
        VerbDictionaryEntry { lemma: "ugryzti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uhoditi" => &[
        VerbDictionaryEntry { lemma: "uhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ujdti" => &[
        VerbDictionaryEntry { lemma: "ujdti", addition: "(ujde; ušėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ujediniti" => &[
        VerbDictionaryEntry { lemma: "ujediniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ujedinjati" => &[
        VerbDictionaryEntry { lemma: "ujedinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ujehati" => &[
        VerbDictionaryEntry { lemma: "ujehati", addition: "(ujede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uježđati" => &[
        VerbDictionaryEntry { lemma: "uježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ujmati" => &[
        VerbDictionaryEntry { lemma: "ujmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ujęti" => &[
        VerbDictionaryEntry { lemma: "ujęti", addition: "(ujme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukazati" => &[
        VerbDictionaryEntry { lemma: "ukazati", addition: "(ukaže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukazyvati" => &[
        VerbDictionaryEntry { lemma: "ukazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uklåti" => &[
        VerbDictionaryEntry { lemma: "uklåti", addition: "(ukolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrasiti" => &[
        VerbDictionaryEntry { lemma: "ukrasiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrasti" => &[
        VerbDictionaryEntry { lemma: "ukrasti", addition: "(ukrade)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrašati" => &[
        VerbDictionaryEntry { lemma: "ukrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrotiti" => &[
        VerbDictionaryEntry { lemma: "ukrotiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukryti" => &[
        VerbDictionaryEntry { lemma: "ukryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukryvati" => &[
        VerbDictionaryEntry { lemma: "ukryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrěpiti" => &[
        VerbDictionaryEntry { lemma: "ukrěpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ukrěpjati" => &[
        VerbDictionaryEntry { lemma: "ukrěpjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ukųsiti" => &[
        VerbDictionaryEntry { lemma: "ukųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ulagađati" => &[
        VerbDictionaryEntry { lemma: "ulagađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ulagoditi" => &[
        VerbDictionaryEntry { lemma: "ulagoditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uletěti" => &[
        VerbDictionaryEntry { lemma: "uletěti", addition: "(uleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ulučati" => &[
        VerbDictionaryEntry { lemma: "ulučati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ulučiti" => &[
        VerbDictionaryEntry { lemma: "ulučiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ulučšati" => &[
        VerbDictionaryEntry { lemma: "ulučšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ulučšiti" => &[
        VerbDictionaryEntry { lemma: "ulučšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ulėgšati" => &[
        VerbDictionaryEntry { lemma: "ulėgšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ulėgšiti" => &[
        VerbDictionaryEntry { lemma: "ulėgšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ulěpšati" => &[
        VerbDictionaryEntry { lemma: "ulěpšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ulěpšiti" => &[
        VerbDictionaryEntry { lemma: "ulěpšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ulětati" => &[
        VerbDictionaryEntry { lemma: "ulětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "umarjati" => &[
        VerbDictionaryEntry { lemma: "umarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umirati" => &[
        VerbDictionaryEntry { lemma: "umirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "umoliti" => &[
        VerbDictionaryEntry { lemma: "umoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "umoljati" => &[
        VerbDictionaryEntry { lemma: "umoljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umoriti" => &[
        VerbDictionaryEntry { lemma: "umoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "umoriti sę" => &[
        VerbDictionaryEntry { lemma: "umoriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "umožniti" => &[
        VerbDictionaryEntry { lemma: "umožniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "umožnjati" => &[
        VerbDictionaryEntry { lemma: "umožnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umrěti" => &[
        VerbDictionaryEntry { lemma: "umrěti", addition: "(umre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "umyti" => &[
        VerbDictionaryEntry { lemma: "umyti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "umyvati" => &[
        VerbDictionaryEntry { lemma: "umyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umėnšati" => &[
        VerbDictionaryEntry { lemma: "umėnšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umėnšiti" => &[
        VerbDictionaryEntry { lemma: "umėnšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uměriti" => &[
        VerbDictionaryEntry { lemma: "uměriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uměrjati" => &[
        VerbDictionaryEntry { lemma: "uměrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uměstiti" => &[
        VerbDictionaryEntry { lemma: "uměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uměti" => &[
        VerbDictionaryEntry { lemma: "uměti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uměšćati" => &[
        VerbDictionaryEntry { lemma: "uměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "umŕti" => &[
        VerbDictionaryEntry { lemma: "umŕti", addition: "(umre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "umŕtviti" => &[
        VerbDictionaryEntry { lemma: "umŕtviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "umŕtvjati" => &[
        VerbDictionaryEntry { lemma: "umŕtvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "unarodniti" => &[
        VerbDictionaryEntry { lemma: "unarodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "unarodnjati" => &[
        VerbDictionaryEntry { lemma: "unarodnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "unemožniti" => &[
        VerbDictionaryEntry { lemma: "unemožniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "unemožnjati" => &[
        VerbDictionaryEntry { lemma: "unemožnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uneviniti" => &[
        VerbDictionaryEntry { lemma: "uneviniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "unevinjati" => &[
        VerbDictionaryEntry { lemma: "unevinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uniziti" => &[
        VerbDictionaryEntry { lemma: "uniziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uniščiti" => &[
        VerbDictionaryEntry { lemma: "uniščiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "unižati" => &[
        VerbDictionaryEntry { lemma: "unižati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upakovati" => &[
        VerbDictionaryEntry { lemma: "upakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upakovyvati" => &[
        VerbDictionaryEntry { lemma: "upakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upasti" => &[
        VerbDictionaryEntry { lemma: "upasti", addition: "(upade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "upekti" => &[
        VerbDictionaryEntry { lemma: "upekti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uperiti" => &[
        VerbDictionaryEntry { lemma: "uperiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upiti sę" => &[
        VerbDictionaryEntry { lemma: "upiti sę", addition: "(upije)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "upivati sę" => &[
        VerbDictionaryEntry { lemma: "upivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "upodabnjati" => &[
        VerbDictionaryEntry { lemma: "upodabnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upodabnjati sę" => &[
        VerbDictionaryEntry { lemma: "upodabnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "upodobniti" => &[
        VerbDictionaryEntry { lemma: "upodobniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upodobniti sę" => &[
        VerbDictionaryEntry { lemma: "upodobniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "upokarnjati" => &[
        VerbDictionaryEntry { lemma: "upokarnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upokorniti" => &[
        VerbDictionaryEntry { lemma: "upokorniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upotrěbiti" => &[
        VerbDictionaryEntry { lemma: "upotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upotrěbjati" => &[
        VerbDictionaryEntry { lemma: "upotrěbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upraviti" => &[
        VerbDictionaryEntry { lemma: "upraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upravjati" => &[
        VerbDictionaryEntry { lemma: "upravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uprašćati" => &[
        VerbDictionaryEntry { lemma: "uprašćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uprostiti" => &[
        VerbDictionaryEntry { lemma: "uprostiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "upȯlnomoćevati" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćevati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "upȯlnomoćiti" => &[
        VerbDictionaryEntry { lemma: "upȯlnomoćiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uraziti" => &[
        VerbDictionaryEntry { lemma: "uraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uražati" => &[
        VerbDictionaryEntry { lemma: "uražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uroditi" => &[
        VerbDictionaryEntry { lemma: "uroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uroditi sę" => &[
        VerbDictionaryEntry { lemma: "uroditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "urvati" => &[
        VerbDictionaryEntry { lemma: "urvati", addition: "(urve)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uryvati" => &[
        VerbDictionaryEntry { lemma: "uryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uråvniti" => &[
        VerbDictionaryEntry { lemma: "uråvniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uråvnjati" => &[
        VerbDictionaryEntry { lemma: "uråvnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uręditi" => &[
        VerbDictionaryEntry { lemma: "uręditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uręđati" => &[
        VerbDictionaryEntry { lemma: "uręđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "urěkati" => &[
        VerbDictionaryEntry { lemma: "urěkati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "urěkti" => &[
        VerbDictionaryEntry { lemma: "urěkti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "urězyvati" => &[
        VerbDictionaryEntry { lemma: "urězyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "usiliti" => &[
        VerbDictionaryEntry { lemma: "usiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "usiljati" => &[
        VerbDictionaryEntry { lemma: "usiljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "usilovati" => &[
        VerbDictionaryEntry { lemma: "usilovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "uskoriti" => &[
        VerbDictionaryEntry { lemma: "uskoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uskorjati" => &[
        VerbDictionaryEntry { lemma: "uskorjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "usložniti" => &[
        VerbDictionaryEntry { lemma: "usložniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "usložnjati" => &[
        VerbDictionaryEntry { lemma: "usložnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uslyšati" => &[
        VerbDictionaryEntry { lemma: "uslyšati", addition: "(uslyši)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "usměhati sę" => &[
        VerbDictionaryEntry { lemma: "usměhati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "usměhnųti sę" => &[
        VerbDictionaryEntry { lemma: "usměhnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "usmŕtiti" => &[
        VerbDictionaryEntry { lemma: "usmŕtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "usmŕćati" => &[
        VerbDictionaryEntry { lemma: "usmŕćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "usnųti" => &[
        VerbDictionaryEntry { lemma: "usnųti", addition: "(usne)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uspokajati" => &[
        VerbDictionaryEntry { lemma: "uspokajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uspokojiti" => &[
        VerbDictionaryEntry { lemma: "uspokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "usposabjati" => &[
        VerbDictionaryEntry { lemma: "usposabjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "usposobiti" => &[
        VerbDictionaryEntry { lemma: "usposobiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uspravědliviti" => &[
        VerbDictionaryEntry { lemma: "uspravědliviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uspravědlivjati" => &[
        VerbDictionaryEntry { lemma: "uspravědlivjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uspěti" => &[
        VerbDictionaryEntry { lemma: "uspěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uspěvati" => &[
        VerbDictionaryEntry { lemma: "uspěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ustaliti" => &[
        VerbDictionaryEntry { lemma: "ustaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ustaljati" => &[
        VerbDictionaryEntry { lemma: "ustaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ustanavjati" => &[
        VerbDictionaryEntry { lemma: "ustanavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ustanoviti" => &[
        VerbDictionaryEntry { lemma: "ustanoviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ustati" => &[
        VerbDictionaryEntry { lemma: "ustati", addition: "(ustane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "ustavati" => &[
        VerbDictionaryEntry { lemma: "ustavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ustaviti" => &[
        VerbDictionaryEntry { lemma: "ustaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ustavjati" => &[
        VerbDictionaryEntry { lemma: "ustavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ustrajati" => &[
        VerbDictionaryEntry { lemma: "ustrajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ustrojiti" => &[
        VerbDictionaryEntry { lemma: "ustrojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ustųpati" => &[
        VerbDictionaryEntry { lemma: "ustųpati", addition: "(+3)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: Some(3) },
        VerbDictionaryEntry { lemma: "ustųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ustųpiti" => &[
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "(+3)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: Some(3) },
        VerbDictionaryEntry { lemma: "ustųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "usyhati" => &[
        VerbDictionaryEntry { lemma: "usyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "usěsti" => &[
        VerbDictionaryEntry { lemma: "usěsti", addition: "(usěde)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "usȯhnųti" => &[
        VerbDictionaryEntry { lemma: "usȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "utekti" => &[
        VerbDictionaryEntry { lemma: "utekti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "utekųćiniti" => &[
        VerbDictionaryEntry { lemma: "utekųćiniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utekųćinjati" => &[
        VerbDictionaryEntry { lemma: "utekųćinjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utirati" => &[
        VerbDictionaryEntry { lemma: "utirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utišati" => &[
        VerbDictionaryEntry { lemma: "utišati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utišiti" => &[
        VerbDictionaryEntry { lemma: "utišiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utonųti" => &[
        VerbDictionaryEntry { lemma: "utonųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "utopiti" => &[
        VerbDictionaryEntry { lemma: "utopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utopiti sę" => &[
        VerbDictionaryEntry { lemma: "utopiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "utratiti" => &[
        VerbDictionaryEntry { lemma: "utratiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utratiti sę" => &[
        VerbDictionaryEntry { lemma: "utratiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "utraćati" => &[
        VerbDictionaryEntry { lemma: "utraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utraćati sę" => &[
        VerbDictionaryEntry { lemma: "utraćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "utrudniti" => &[
        VerbDictionaryEntry { lemma: "utrudniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utrudnjati" => &[
        VerbDictionaryEntry { lemma: "utrudnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utrěti" => &[
        VerbDictionaryEntry { lemma: "utrěti", addition: "(utre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utvŕditi" => &[
        VerbDictionaryEntry { lemma: "utvŕditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utvŕđati" => &[
        VerbDictionaryEntry { lemma: "utvŕđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utęžati" => &[
        VerbDictionaryEntry { lemma: "utęžati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utęžiti" => &[
        VerbDictionaryEntry { lemma: "utęžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utěkati" => &[
        VerbDictionaryEntry { lemma: "utěkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "utělesniti" => &[
        VerbDictionaryEntry { lemma: "utělesniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utělesnjati" => &[
        VerbDictionaryEntry { lemma: "utělesnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utěšati" => &[
        VerbDictionaryEntry { lemma: "utěšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "utěšiti" => &[
        VerbDictionaryEntry { lemma: "utěšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utŕpěti" => &[
        VerbDictionaryEntry { lemma: "utŕpěti", addition: "(utŕpi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utŕti" => &[
        VerbDictionaryEntry { lemma: "utŕti", addition: "(utre)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utȯčniti" => &[
        VerbDictionaryEntry { lemma: "utȯčniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "utȯčnjati" => &[
        VerbDictionaryEntry { lemma: "utȯčnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvadnjati" => &[
        VerbDictionaryEntry { lemma: "uvadnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvaljnjati" => &[
        VerbDictionaryEntry { lemma: "uvaljnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvaljnjati sę" => &[
        VerbDictionaryEntry { lemma: "uvaljnjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "uvažati" => &[
        VerbDictionaryEntry { lemma: "uvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvažiti" => &[
        VerbDictionaryEntry { lemma: "uvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uveličati" => &[
        VerbDictionaryEntry { lemma: "uveličati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uveličiti" => &[
        VerbDictionaryEntry { lemma: "uveličiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uviděti" => &[
        VerbDictionaryEntry { lemma: "uviděti", addition: "(uvidi)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvodniti" => &[
        VerbDictionaryEntry { lemma: "uvodniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvoljniti" => &[
        VerbDictionaryEntry { lemma: "uvoljniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvoljniti sę" => &[
        VerbDictionaryEntry { lemma: "uvoljniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "uvędati" => &[
        VerbDictionaryEntry { lemma: "uvędati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "uvędnųti" => &[
        VerbDictionaryEntry { lemma: "uvędnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uvęznųti" => &[
        VerbDictionaryEntry { lemma: "uvęznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "uvědamjati" => &[
        VerbDictionaryEntry { lemma: "uvědamjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvědomiti" => &[
        VerbDictionaryEntry { lemma: "uvědomiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvěkověčiti" => &[
        VerbDictionaryEntry { lemma: "uvěkověčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvěriti" => &[
        VerbDictionaryEntry { lemma: "uvěriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uvěriti sę" => &[
        VerbDictionaryEntry { lemma: "uvěriti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "uvěrjati" => &[
        VerbDictionaryEntry { lemma: "uvěrjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uvěrjati sę" => &[
        VerbDictionaryEntry { lemma: "uvěrjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "uzdravjati" => &[
        VerbDictionaryEntry { lemma: "uzdravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uzdråviti" => &[
        VerbDictionaryEntry { lemma: "uzdråviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uznati" => &[
        VerbDictionaryEntry { lemma: "uznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uznavati" => &[
        VerbDictionaryEntry { lemma: "uznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "uzrěti" => &[
        VerbDictionaryEntry { lemma: "uzrěti", addition: "(uzri)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uzurpovati" => &[
        VerbDictionaryEntry { lemma: "uzurpovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "učarovati" => &[
        VerbDictionaryEntry { lemma: "učarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "učarovyvati" => &[
        VerbDictionaryEntry { lemma: "učarovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "učiti" => &[
        VerbDictionaryEntry { lemma: "učiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "učiti sę" => &[
        VerbDictionaryEntry { lemma: "učiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "učęstvovati" => &[
        VerbDictionaryEntry { lemma: "učęstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "užasati" => &[
        VerbDictionaryEntry { lemma: "užasati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "užasnųti" => &[
        VerbDictionaryEntry { lemma: "užasnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "užiti" => &[
        VerbDictionaryEntry { lemma: "užiti", addition: "(užive)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "uživati" => &[
        VerbDictionaryEntry { lemma: "uživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vabiti" => &[
        VerbDictionaryEntry { lemma: "vabiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vagati sę" => &[
        VerbDictionaryEntry { lemma: "vagati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vajati" => &[
        VerbDictionaryEntry { lemma: "vajati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "valiti" => &[
        VerbDictionaryEntry { lemma: "valiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "valjati" => &[
        VerbDictionaryEntry { lemma: "valjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "valjati sę" => &[
        VerbDictionaryEntry { lemma: "valjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "valjcevati" => &[
        VerbDictionaryEntry { lemma: "valjcevati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "valsovati" => &[
        VerbDictionaryEntry { lemma: "valsovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "variovati" => &[
        VerbDictionaryEntry { lemma: "variovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "variti" => &[
        VerbDictionaryEntry { lemma: "variti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "važiti" => &[
        VerbDictionaryEntry { lemma: "važiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vdyhati" => &[
        VerbDictionaryEntry { lemma: "vdyhati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vdȯhnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "velěti" => &[
        VerbDictionaryEntry { lemma: "velěti", addition: "(veli)", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ventilovati" => &[
        VerbDictionaryEntry { lemma: "ventilovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "verbovati" => &[
        VerbDictionaryEntry { lemma: "verbovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "verifikovati" => &[
        VerbDictionaryEntry { lemma: "verifikovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "veseliti" => &[
        VerbDictionaryEntry { lemma: "veseliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "veseliti sę" => &[
        VerbDictionaryEntry { lemma: "veseliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vesti" => &[
        VerbDictionaryEntry { lemma: "vesti", addition: "(vede)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vezti" => &[
        VerbDictionaryEntry { lemma: "vezti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vezti sę" => &[
        VerbDictionaryEntry { lemma: "vezti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "večerjati" => &[
        VerbDictionaryEntry { lemma: "večerjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vględati sę" => &[
        VerbDictionaryEntry { lemma: "vględati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vględěti sę" => &[
        VerbDictionaryEntry { lemma: "vględěti sę", addition: "(vględi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vhoditi" => &[
        VerbDictionaryEntry { lemma: "vhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vibrovati" => &[
        VerbDictionaryEntry { lemma: "vibrovati", addition: "", transitive: false, imperfective: true, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "viděti" => &[
        VerbDictionaryEntry { lemma: "viděti", addition: "(vidi)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "visěti" => &[
        VerbDictionaryEntry { lemma: "visěti", addition: "(visi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vitati" => &[
        VerbDictionaryEntry { lemma: "vitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "viti" => &[
        VerbDictionaryEntry { lemma: "viti", addition: "(vije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "viti sę" => &[
        VerbDictionaryEntry { lemma: "viti sę", addition: "(vije)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vjehati" => &[
        VerbDictionaryEntry { lemma: "vjehati", addition: "(vjede)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vježđati" => &[
        VerbDictionaryEntry { lemma: "vježđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vkladati" => &[
        VerbDictionaryEntry { lemma: "vkladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vključati" => &[
        VerbDictionaryEntry { lemma: "vključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vključiti" => &[
        VerbDictionaryEntry { lemma: "vključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vkųsiti" => &[
        VerbDictionaryEntry { lemma: "vkųsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vkųšati" => &[
        VerbDictionaryEntry { lemma: "vkųšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vladati" => &[
        VerbDictionaryEntry { lemma: "vladati", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(5) },
    ],
    "vlagati" => &[
        VerbDictionaryEntry { lemma: "vlagati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vlamyvati sę" => &[
        VerbDictionaryEntry { lemma: "vlamyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vlastniti" => &[
        VerbDictionaryEntry { lemma: "vlastniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vlivati" => &[
        VerbDictionaryEntry { lemma: "vlivati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vlomiti sę" => &[
        VerbDictionaryEntry { lemma: "vlomiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vložiti" => &[
        VerbDictionaryEntry { lemma: "vložiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vlåděti" => &[
        VerbDictionaryEntry { lemma: "vlåděti", addition: "(+5)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: Some(5) },
    ],
    "vlåčiti" => &[
        VerbDictionaryEntry { lemma: "vlåčiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vlåžiti" => &[
        VerbDictionaryEntry { lemma: "vlåžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vlěkti" => &[
        VerbDictionaryEntry { lemma: "vlěkti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vlězati" => &[
        VerbDictionaryEntry { lemma: "vlězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vměstiti" => &[
        VerbDictionaryEntry { lemma: "vměstiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vměšati" => &[
        VerbDictionaryEntry { lemma: "vměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vměšati sę" => &[
        VerbDictionaryEntry { lemma: "vměšati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vměšivati" => &[
        VerbDictionaryEntry { lemma: "vměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vměšivati sę" => &[
        VerbDictionaryEntry { lemma: "vměšivati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vměšćati" => &[
        VerbDictionaryEntry { lemma: "vměšćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vnesti" => &[
        VerbDictionaryEntry { lemma: "vnesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vnikati" => &[
        VerbDictionaryEntry { lemma: "vnikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vniknųti" => &[
        VerbDictionaryEntry { lemma: "vniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vnositi" => &[
        VerbDictionaryEntry { lemma: "vnositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "voditi" => &[
        VerbDictionaryEntry { lemma: "voditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vojevati" => &[
        VerbDictionaryEntry { lemma: "vojevati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "volěti" => &[
        VerbDictionaryEntry { lemma: "volěti", addition: "(voli)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vonjati" => &[
        VerbDictionaryEntry { lemma: "vonjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "voziti" => &[
        VerbDictionaryEntry { lemma: "voziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "voziti sę" => &[
        VerbDictionaryEntry { lemma: "voziti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "voščiti" => &[
        VerbDictionaryEntry { lemma: "voščiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vpadati" => &[
        VerbDictionaryEntry { lemma: "vpadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vpasti" => &[
        VerbDictionaryEntry { lemma: "vpasti", addition: "(vpade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vpihati" => &[
        VerbDictionaryEntry { lemma: "vpihati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vpisati" => &[
        VerbDictionaryEntry { lemma: "vpisati", addition: "(vpiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vpisyvati" => &[
        VerbDictionaryEntry { lemma: "vpisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vpiti" => &[
        VerbDictionaryEntry { lemma: "vpiti", addition: "(vpije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vpivati" => &[
        VerbDictionaryEntry { lemma: "vpivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vplesti" => &[
        VerbDictionaryEntry { lemma: "vplesti", addition: "(vplete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vpletati" => &[
        VerbDictionaryEntry { lemma: "vpletati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vplyvati" => &[
        VerbDictionaryEntry { lemma: "vplyvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vplyvti" => &[
        VerbDictionaryEntry { lemma: "vplyvti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vpręgati" => &[
        VerbDictionaryEntry { lemma: "vpręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vpręgti" => &[
        VerbDictionaryEntry { lemma: "vpręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vraćati" => &[
        VerbDictionaryEntry { lemma: "vraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vraćati sę" => &[
        VerbDictionaryEntry { lemma: "vraćati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vråtiti" => &[
        VerbDictionaryEntry { lemma: "vråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vråtiti sę" => &[
        VerbDictionaryEntry { lemma: "vråtiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vråžiti" => &[
        VerbDictionaryEntry { lemma: "vråžiti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vrčati" => &[
        VerbDictionaryEntry { lemma: "vrčati", addition: "(vrči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vrěti" => &[
        VerbDictionaryEntry { lemma: "vrěti", addition: "(vri)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vrěščati" => &[
        VerbDictionaryEntry { lemma: "vrěščati", addition: "(vrěšči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vrųbati" => &[
        VerbDictionaryEntry { lemma: "vrųbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vrųbyvati" => &[
        VerbDictionaryEntry { lemma: "vrųbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vrųčati" => &[
        VerbDictionaryEntry { lemma: "vrųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vrųčiti" => &[
        VerbDictionaryEntry { lemma: "vrųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vsaditi" => &[
        VerbDictionaryEntry { lemma: "vsaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vsađati" => &[
        VerbDictionaryEntry { lemma: "vsađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vskočiti" => &[
        VerbDictionaryEntry { lemma: "vskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vstati" => &[
        VerbDictionaryEntry { lemma: "vstati", addition: "(vstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vstavati" => &[
        VerbDictionaryEntry { lemma: "vstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vstaviti" => &[
        VerbDictionaryEntry { lemma: "vstaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vstavjati" => &[
        VerbDictionaryEntry { lemma: "vstavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vstųpati" => &[
        VerbDictionaryEntry { lemma: "vstųpati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vstųpiti" => &[
        VerbDictionaryEntry { lemma: "vstųpiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vtiskati" => &[
        VerbDictionaryEntry { lemma: "vtiskati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vtisknųti" => &[
        VerbDictionaryEntry { lemma: "vtisknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vtrgati" => &[
        VerbDictionaryEntry { lemma: "vtrgati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vtrgnųti" => &[
        VerbDictionaryEntry { lemma: "vtrgnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vtęgati" => &[
        VerbDictionaryEntry { lemma: "vtęgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vtęgnųti" => &[
        VerbDictionaryEntry { lemma: "vtęgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vulkanizovati" => &[
        VerbDictionaryEntry { lemma: "vulkanizovati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vvesti" => &[
        VerbDictionaryEntry { lemma: "vvesti", addition: "(vvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vvezti" => &[
        VerbDictionaryEntry { lemma: "vvezti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vvoditi" => &[
        VerbDictionaryEntry { lemma: "vvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vvoziti" => &[
        VerbDictionaryEntry { lemma: "vvoziti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vvŕgati" => &[
        VerbDictionaryEntry { lemma: "vvŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vvŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vvŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vyjdti" => &[
        VerbDictionaryEntry { lemma: "vyjdti", addition: "(vyjde; vyšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vyjmati" => &[
        VerbDictionaryEntry { lemma: "vyjmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vyryti" => &[
        VerbDictionaryEntry { lemma: "vyryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vyti" => &[
        VerbDictionaryEntry { lemma: "vyti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vzajėmodějati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějati", addition: "(vzajemoděje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vzajėmodějstvovati" => &[
        VerbDictionaryEntry { lemma: "vzajėmodějstvovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vzęti" => &[
        VerbDictionaryEntry { lemma: "vzęti", addition: "(vȯzme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "včleniti" => &[
        VerbDictionaryEntry { lemma: "včleniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "včlenjati" => &[
        VerbDictionaryEntry { lemma: "včlenjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vędnųti" => &[
        VerbDictionaryEntry { lemma: "vędnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vęzati" => &[
        VerbDictionaryEntry { lemma: "vęzati", addition: "(vęže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vęznųti" => &[
        VerbDictionaryEntry { lemma: "vęznųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "věděti" => &[
        VerbDictionaryEntry { lemma: "věděti", addition: "(vě)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vějati" => &[
        VerbDictionaryEntry { lemma: "vějati", addition: "(věje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "věriti" => &[
        VerbDictionaryEntry { lemma: "věriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "větriti" => &[
        VerbDictionaryEntry { lemma: "větriti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "věšati" => &[
        VerbDictionaryEntry { lemma: "věšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vŕgati" => &[
        VerbDictionaryEntry { lemma: "vŕgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vŕgnųti" => &[
        VerbDictionaryEntry { lemma: "vŕgnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vŕnųti" => &[
        VerbDictionaryEntry { lemma: "vŕnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vŕnųti sę" => &[
        VerbDictionaryEntry { lemma: "vŕnųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vŕtěti" => &[
        VerbDictionaryEntry { lemma: "vŕtěti", addition: "(vŕti)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
        VerbDictionaryEntry { lemma: "vŕtěti", addition: "(vŕti)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vžiti sę" => &[
        VerbDictionaryEntry { lemma: "vžiti sę", addition: "(vžive)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vživati sę" => &[
        VerbDictionaryEntry { lemma: "vživati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯjdti", addition: "(vȯjde; všėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯlgnųti" => &[
        VerbDictionaryEntry { lemma: "vȯlgnųti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯorųžiti" => &[
        VerbDictionaryEntry { lemma: "vȯorųžiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯphati" => &[
        VerbDictionaryEntry { lemma: "vȯphati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯplȯtiti" => &[
        VerbDictionaryEntry { lemma: "vȯplȯtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯplȯćati" => &[
        VerbDictionaryEntry { lemma: "vȯplȯćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbogatiti" => &[
        VerbDictionaryEntry { lemma: "vȯzbogatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbogaćati" => &[
        VerbDictionaryEntry { lemma: "vȯzbogaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbranjati" => &[
        VerbDictionaryEntry { lemma: "vȯzbranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbråniti" => &[
        VerbDictionaryEntry { lemma: "vȯzbråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbuditi" => &[
        VerbDictionaryEntry { lemma: "vȯzbuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzbuđati" => &[
        VerbDictionaryEntry { lemma: "vȯzbuđati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzdvignųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdvignųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzdyhati" => &[
        VerbDictionaryEntry { lemma: "vȯzdyhati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzdŕžati sę" => &[
        VerbDictionaryEntry { lemma: "vȯzdŕžati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "vȯzdŕživati sę" => &[
        VerbDictionaryEntry { lemma: "vȯzdŕživati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "vȯzdȯhnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzdȯhnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzhoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzhoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzimati" => &[
        VerbDictionaryEntry { lemma: "vȯzimati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzjęti" => &[
        VerbDictionaryEntry { lemma: "vȯzjęti", addition: "(vȯzȯjme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzklicati" => &[
        VerbDictionaryEntry { lemma: "vȯzklicati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzkliknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkliknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzkresati" => &[
        VerbDictionaryEntry { lemma: "vȯzkresati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzkresiti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzkresnųti" => &[
        VerbDictionaryEntry { lemma: "vȯzkresnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzkrešati" => &[
        VerbDictionaryEntry { lemma: "vȯzkrešati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzletěti" => &[
        VerbDictionaryEntry { lemma: "vȯzletěti", addition: "(vȯzleti)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzlětati" => &[
        VerbDictionaryEntry { lemma: "vȯzlětati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯznesti" => &[
        VerbDictionaryEntry { lemma: "vȯznesti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯznikati" => &[
        VerbDictionaryEntry { lemma: "vȯznikati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzniknųti" => &[
        VerbDictionaryEntry { lemma: "vȯzniknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯznositi" => &[
        VerbDictionaryEntry { lemma: "vȯznositi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzpamętati" => &[
        VerbDictionaryEntry { lemma: "vȯzpamętati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzpitati" => &[
        VerbDictionaryEntry { lemma: "vȯzpitati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzpityvati" => &[
        VerbDictionaryEntry { lemma: "vȯzpityvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzpominati" => &[
        VerbDictionaryEntry { lemma: "vȯzpominati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzpomněti" => &[
        VerbDictionaryEntry { lemma: "vȯzpomněti", addition: "(vȯzpomni)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzrastati" => &[
        VerbDictionaryEntry { lemma: "vȯzrastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzroditi" => &[
        VerbDictionaryEntry { lemma: "vȯzroditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzråsti" => &[
        VerbDictionaryEntry { lemma: "vȯzråsti", addition: "(vȯzråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzsiliti" => &[
        VerbDictionaryEntry { lemma: "vȯzsiliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzsilovati" => &[
        VerbDictionaryEntry { lemma: "vȯzsilovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzskakati" => &[
        VerbDictionaryEntry { lemma: "vȯzskakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzskočiti" => &[
        VerbDictionaryEntry { lemma: "vȯzskočiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzstati" => &[
        VerbDictionaryEntry { lemma: "vȯzstati", addition: "(vȯzstane)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯzstavati" => &[
        VerbDictionaryEntry { lemma: "vȯzstavati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "vȯztręsati" => &[
        VerbDictionaryEntry { lemma: "vȯztręsati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯztręsti" => &[
        VerbDictionaryEntry { lemma: "vȯztręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzveličati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzveličivati" => &[
        VerbDictionaryEntry { lemma: "vȯzveličivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvesti" => &[
        VerbDictionaryEntry { lemma: "vȯzvesti", addition: "(vȯzvede)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvoditi" => &[
        VerbDictionaryEntry { lemma: "vȯzvoditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvraćati" => &[
        VerbDictionaryEntry { lemma: "vȯzvraćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvråtiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvråtiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvysiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvysiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvyšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvyšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvŕšati" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzvŕšiti" => &[
        VerbDictionaryEntry { lemma: "vȯzvŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "vȯzȯjdti" => &[
        VerbDictionaryEntry { lemma: "vȯzȯjdti", addition: "(vȯzȯjde; vȯzšėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zabarikadovati" => &[
        VerbDictionaryEntry { lemma: "zabarikadovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabaviti" => &[
        VerbDictionaryEntry { lemma: "zabaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabavjati" => &[
        VerbDictionaryEntry { lemma: "zabavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zabezpamętiti" => &[
        VerbDictionaryEntry { lemma: "zabezpamętiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabezpamęćati" => &[
        VerbDictionaryEntry { lemma: "zabezpamęćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zabezpečati" => &[
        VerbDictionaryEntry { lemma: "zabezpečati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zabezpečiti" => &[
        VerbDictionaryEntry { lemma: "zabezpečiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabirati" => &[
        VerbDictionaryEntry { lemma: "zabirati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zablokovati" => &[
        VerbDictionaryEntry { lemma: "zablokovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zablųditi" => &[
        VerbDictionaryEntry { lemma: "zablųditi", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zablųđati" => &[
        VerbDictionaryEntry { lemma: "zablųđati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zabolěti" => &[
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zabolěje)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "zabolěti", addition: "(zaboli)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zabolěvati" => &[
        VerbDictionaryEntry { lemma: "zabolěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zabranjati" => &[
        VerbDictionaryEntry { lemma: "zabranjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zabrati" => &[
        VerbDictionaryEntry { lemma: "zabrati", addition: "(zabere)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabråniti" => &[
        VerbDictionaryEntry { lemma: "zabråniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabyti" => &[
        VerbDictionaryEntry { lemma: "zabyti", addition: "(zabųde)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zabyvati" => &[
        VerbDictionaryEntry { lemma: "zabyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zadati" => &[
        VerbDictionaryEntry { lemma: "zadati", addition: "(zada)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zadavati" => &[
        VerbDictionaryEntry { lemma: "zadavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zadovaljati" => &[
        VerbDictionaryEntry { lemma: "zadovaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zadovaljati sę" => &[
        VerbDictionaryEntry { lemma: "zadovaljati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zadovoliti" => &[
        VerbDictionaryEntry { lemma: "zadovoliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zadovoliti sę" => &[
        VerbDictionaryEntry { lemma: "zadovoliti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zadrěmati" => &[
        VerbDictionaryEntry { lemma: "zadrěmati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zadržati" => &[
        VerbDictionaryEntry { lemma: "zadržati", addition: "(zadŕži)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zadŕžati" => &[
        VerbDictionaryEntry { lemma: "zadŕžati", addition: "(zadŕži)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zadŕžati sę" => &[
        VerbDictionaryEntry { lemma: "zadŕžati sę", addition: "(zadŕži)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zadŕživati" => &[
        VerbDictionaryEntry { lemma: "zadŕživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zadŕživati sę" => &[
        VerbDictionaryEntry { lemma: "zadŕživati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zagladiti" => &[
        VerbDictionaryEntry { lemma: "zagladiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaglađati" => &[
        VerbDictionaryEntry { lemma: "zaglađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zagorěti" => &[
        VerbDictionaryEntry { lemma: "zagorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zagospodariti" => &[
        VerbDictionaryEntry { lemma: "zagospodariti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zagroziti" => &[
        VerbDictionaryEntry { lemma: "zagroziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zahoditi" => &[
        VerbDictionaryEntry { lemma: "zahoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zahvatiti" => &[
        VerbDictionaryEntry { lemma: "zahvatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zahvaćati" => &[
        VerbDictionaryEntry { lemma: "zahvaćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zahvorěti" => &[
        VerbDictionaryEntry { lemma: "zahvorěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zahvorěvati" => &[
        VerbDictionaryEntry { lemma: "zahvorěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zainteresovati" => &[
        VerbDictionaryEntry { lemma: "zainteresovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zainteresovati sę" => &[
        VerbDictionaryEntry { lemma: "zainteresovati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zajdti" => &[
        VerbDictionaryEntry { lemma: "zajdti", addition: "(zajde; zašėl)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zajikati sę" => &[
        VerbDictionaryEntry { lemma: "zajikati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zajmati" => &[
        VerbDictionaryEntry { lemma: "zajmati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zajmati sę" => &[
        VerbDictionaryEntry { lemma: "zajmati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zajęti" => &[
        VerbDictionaryEntry { lemma: "zajęti", addition: "(zajme)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zajęti sę" => &[
        VerbDictionaryEntry { lemma: "zajęti sę", addition: "(zajme)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zakazyvati" => &[
        VerbDictionaryEntry { lemma: "zakazyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zakašljati" => &[
        VerbDictionaryEntry { lemma: "zakašljati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zakladati" => &[
        VerbDictionaryEntry { lemma: "zakladati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaklinati" => &[
        VerbDictionaryEntry { lemma: "zaklinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zakliniti" => &[
        VerbDictionaryEntry { lemma: "zakliniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaklinovati" => &[
        VerbDictionaryEntry { lemma: "zaklinovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaključati" => &[
        VerbDictionaryEntry { lemma: "zaključati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "zaključati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaključiti" => &[
        VerbDictionaryEntry { lemma: "zaključiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "zaključiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaklåti" => &[
        VerbDictionaryEntry { lemma: "zaklåti", addition: "(zakolje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaklęti" => &[
        VerbDictionaryEntry { lemma: "zaklęti", addition: "(zaklne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zakončiti" => &[
        VerbDictionaryEntry { lemma: "zakončiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zakopati" => &[
        VerbDictionaryEntry { lemma: "zakopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zakopyvati" => &[
        VerbDictionaryEntry { lemma: "zakopyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zakoreniti sę" => &[
        VerbDictionaryEntry { lemma: "zakoreniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zakričati" => &[
        VerbDictionaryEntry { lemma: "zakričati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zakryti" => &[
        VerbDictionaryEntry { lemma: "zakryti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zakryvati" => &[
        VerbDictionaryEntry { lemma: "zakryvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zalajati" => &[
        VerbDictionaryEntry { lemma: "zalajati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zaljubiti sę" => &[
        VerbDictionaryEntry { lemma: "zaljubiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "založiti" => &[
        VerbDictionaryEntry { lemma: "založiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zalězati" => &[
        VerbDictionaryEntry { lemma: "zalězati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zalězti" => &[
        VerbDictionaryEntry { lemma: "zalězti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zamesti" => &[
        VerbDictionaryEntry { lemma: "zamesti", addition: "(zamete)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamglati" => &[
        VerbDictionaryEntry { lemma: "zamglati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamirati" => &[
        VerbDictionaryEntry { lemma: "zamirati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zamknųti" => &[
        VerbDictionaryEntry { lemma: "zamknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamražati" => &[
        VerbDictionaryEntry { lemma: "zamražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zamrzati" => &[
        VerbDictionaryEntry { lemma: "zamrzati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zamrznųti" => &[
        VerbDictionaryEntry { lemma: "zamrznųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zamråziti" => &[
        VerbDictionaryEntry { lemma: "zamråziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamrěti" => &[
        VerbDictionaryEntry { lemma: "zamrěti", addition: "(zamre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zamykati" => &[
        VerbDictionaryEntry { lemma: "zamykati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zamysliti" => &[
        VerbDictionaryEntry { lemma: "zamysliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamysljati" => &[
        VerbDictionaryEntry { lemma: "zamysljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zamėdliti" => &[
        VerbDictionaryEntry { lemma: "zamėdliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamėdljati" => &[
        VerbDictionaryEntry { lemma: "zamėdljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaměniti" => &[
        VerbDictionaryEntry { lemma: "zaměniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaměnjati" => &[
        VerbDictionaryEntry { lemma: "zaměnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaměsiti" => &[
        VerbDictionaryEntry { lemma: "zaměsiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamětati" => &[
        VerbDictionaryEntry { lemma: "zamětati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaměšati" => &[
        VerbDictionaryEntry { lemma: "zaměšati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaměšivati" => &[
        VerbDictionaryEntry { lemma: "zaměšivati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zamŕti" => &[
        VerbDictionaryEntry { lemma: "zamŕti", addition: "(zamre)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zamȯlknųti" => &[
        VerbDictionaryEntry { lemma: "zamȯlknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zamȯlviti" => &[
        VerbDictionaryEntry { lemma: "zamȯlviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zamȯlvjati" => &[
        VerbDictionaryEntry { lemma: "zamȯlvjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zanedbati" => &[
        VerbDictionaryEntry { lemma: "zanedbati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zanedbati sę" => &[
        VerbDictionaryEntry { lemma: "zanedbati sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zanedbyvati" => &[
        VerbDictionaryEntry { lemma: "zanedbyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zanedbyvati sę" => &[
        VerbDictionaryEntry { lemma: "zanedbyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zanepokojiti" => &[
        VerbDictionaryEntry { lemma: "zanepokojiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zanuriti" => &[
        VerbDictionaryEntry { lemma: "zanuriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zanurjati" => &[
        VerbDictionaryEntry { lemma: "zanurjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapadati" => &[
        VerbDictionaryEntry { lemma: "zapadati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zapakovati" => &[
        VerbDictionaryEntry { lemma: "zapakovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapakovyvati" => &[
        VerbDictionaryEntry { lemma: "zapakovyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapaliti" => &[
        VerbDictionaryEntry { lemma: "zapaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapaljati" => &[
        VerbDictionaryEntry { lemma: "zapaljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapamętati" => &[
        VerbDictionaryEntry { lemma: "zapamętati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapamętyvati" => &[
        VerbDictionaryEntry { lemma: "zapamętyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapasti" => &[
        VerbDictionaryEntry { lemma: "zapasti", addition: "(zapade)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zapečatati" => &[
        VerbDictionaryEntry { lemma: "zapečatati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapečatyvati" => &[
        VerbDictionaryEntry { lemma: "zapečatyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapisati" => &[
        VerbDictionaryEntry { lemma: "zapisati", addition: "(zapiše)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapisati sę" => &[
        VerbDictionaryEntry { lemma: "zapisati sę", addition: "(zapiše)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zapisyvati" => &[
        VerbDictionaryEntry { lemma: "zapisyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapisyvati sę" => &[
        VerbDictionaryEntry { lemma: "zapisyvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zaplakati" => &[
        VerbDictionaryEntry { lemma: "zaplakati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zaplanovati" => &[
        VerbDictionaryEntry { lemma: "zaplanovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaplatiti" => &[
        VerbDictionaryEntry { lemma: "zaplatiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaplěniti" => &[
        VerbDictionaryEntry { lemma: "zaplěniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaplěnjati" => &[
        VerbDictionaryEntry { lemma: "zaplěnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapoznati" => &[
        VerbDictionaryEntry { lemma: "zapoznati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapoznavati" => &[
        VerbDictionaryEntry { lemma: "zapoznavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "započinati" => &[
        VerbDictionaryEntry { lemma: "započinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "započęti" => &[
        VerbDictionaryEntry { lemma: "započęti", addition: "(započne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapraviti" => &[
        VerbDictionaryEntry { lemma: "zapraviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapravjati" => &[
        VerbDictionaryEntry { lemma: "zapravjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaprojektovati" => &[
        VerbDictionaryEntry { lemma: "zaprojektovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapropastiti sę" => &[
        VerbDictionaryEntry { lemma: "zapropastiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zapråšiti" => &[
        VerbDictionaryEntry { lemma: "zapråšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapręgati" => &[
        VerbDictionaryEntry { lemma: "zapręgati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapręgti" => &[
        VerbDictionaryEntry { lemma: "zapręgti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaprěčiti" => &[
        VerbDictionaryEntry { lemma: "zaprěčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapustiti" => &[
        VerbDictionaryEntry { lemma: "zapustiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapušćati" => &[
        VerbDictionaryEntry { lemma: "zapušćati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapytati" => &[
        VerbDictionaryEntry { lemma: "zapytati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapytyvati" => &[
        VerbDictionaryEntry { lemma: "zapytyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zapȯlniti" => &[
        VerbDictionaryEntry { lemma: "zapȯlniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zapȯlnjati" => &[
        VerbDictionaryEntry { lemma: "zapȯlnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zarastati" => &[
        VerbDictionaryEntry { lemma: "zarastati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zaraziti" => &[
        VerbDictionaryEntry { lemma: "zaraziti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaražati" => &[
        VerbDictionaryEntry { lemma: "zaražati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaregistrovati" => &[
        VerbDictionaryEntry { lemma: "zaregistrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zarevti" => &[
        VerbDictionaryEntry { lemma: "zarevti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zarezervovati" => &[
        VerbDictionaryEntry { lemma: "zarezervovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaroditi sę" => &[
        VerbDictionaryEntry { lemma: "zaroditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zarydati" => &[
        VerbDictionaryEntry { lemma: "zarydati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zaråbotati" => &[
        VerbDictionaryEntry { lemma: "zaråbotati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaråbotyvati" => &[
        VerbDictionaryEntry { lemma: "zaråbotyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaråsti" => &[
        VerbDictionaryEntry { lemma: "zaråsti", addition: "(zaråste)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zarěkati sę" => &[
        VerbDictionaryEntry { lemma: "zarěkati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zarěkti sę" => &[
        VerbDictionaryEntry { lemma: "zarěkti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zarųčati" => &[
        VerbDictionaryEntry { lemma: "zarųčati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zarųčati sę" => &[
        VerbDictionaryEntry { lemma: "zarųčati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zarųčiti" => &[
        VerbDictionaryEntry { lemma: "zarųčiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zarųčiti sę" => &[
        VerbDictionaryEntry { lemma: "zarųčiti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zasaditi" => &[
        VerbDictionaryEntry { lemma: "zasaditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zasađati" => &[
        VerbDictionaryEntry { lemma: "zasađati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaslanjati" => &[
        VerbDictionaryEntry { lemma: "zaslanjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zasloniti" => &[
        VerbDictionaryEntry { lemma: "zasloniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaslužiti" => &[
        VerbDictionaryEntry { lemma: "zaslužiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zasluživati" => &[
        VerbDictionaryEntry { lemma: "zasluživati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zasmějati sę" => &[
        VerbDictionaryEntry { lemma: "zasmějati sę", addition: "(+2)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: Some(2) },
    ],
    "zasnųti" => &[
        VerbDictionaryEntry { lemma: "zasnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zaspati" => &[
        VerbDictionaryEntry { lemma: "zaspati", addition: "(zaspi)", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zaspati sę" => &[
        VerbDictionaryEntry { lemma: "zaspati sę", addition: "(zaspi)", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zasramjati" => &[
        VerbDictionaryEntry { lemma: "zasramjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zasrati" => &[
        VerbDictionaryEntry { lemma: "zasrati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zasråmiti" => &[
        VerbDictionaryEntry { lemma: "zasråmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zastariti" => &[
        VerbDictionaryEntry { lemma: "zastariti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zastarjati" => &[
        VerbDictionaryEntry { lemma: "zastarjati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zastati" => &[
        VerbDictionaryEntry { lemma: "zastati", addition: "(zastane)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zastavati" => &[
        VerbDictionaryEntry { lemma: "zastavati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zastaviti" => &[
        VerbDictionaryEntry { lemma: "zastaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zastavjati" => &[
        VerbDictionaryEntry { lemma: "zastavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zastrašati" => &[
        VerbDictionaryEntry { lemma: "zastrašati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zastrašiti" => &[
        VerbDictionaryEntry { lemma: "zastrašiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zastrěliti" => &[
        VerbDictionaryEntry { lemma: "zastrěliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zastrěljati" => &[
        VerbDictionaryEntry { lemma: "zastrěljati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zastųpati" => &[
        VerbDictionaryEntry { lemma: "zastųpati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zastųpiti" => &[
        VerbDictionaryEntry { lemma: "zastųpiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zasvěćati" => &[
        VerbDictionaryEntry { lemma: "zasvěćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zasějati" => &[
        VerbDictionaryEntry { lemma: "zasějati", addition: "(zasěje)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatelefonovati" => &[
        VerbDictionaryEntry { lemma: "zatelefonovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zatknųti" => &[
        VerbDictionaryEntry { lemma: "zatknųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatknųti sę" => &[
        VerbDictionaryEntry { lemma: "zatknųti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zatopiti" => &[
        VerbDictionaryEntry { lemma: "zatopiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatrimati" => &[
        VerbDictionaryEntry { lemma: "zatrimati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatrimyvati" => &[
        VerbDictionaryEntry { lemma: "zatrimyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zatręsti" => &[
        VerbDictionaryEntry { lemma: "zatręsti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatvarjati" => &[
        VerbDictionaryEntry { lemma: "zatvarjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zatvoriti" => &[
        VerbDictionaryEntry { lemma: "zatvoriti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatykati" => &[
        VerbDictionaryEntry { lemma: "zatykati", addition: "(zatyče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zatykati sę" => &[
        VerbDictionaryEntry { lemma: "zatykati sę", addition: "(zatyče)", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zatėmniti" => &[
        VerbDictionaryEntry { lemma: "zatėmniti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zatėmnjati" => &[
        VerbDictionaryEntry { lemma: "zatėmnjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaustaviti" => &[
        VerbDictionaryEntry { lemma: "zaustaviti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaustaviti sę" => &[
        VerbDictionaryEntry { lemma: "zaustaviti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "zaustavjati" => &[
        VerbDictionaryEntry { lemma: "zaustavjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaustavjati sę" => &[
        VerbDictionaryEntry { lemma: "zaustavjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zautrakati" => &[
        VerbDictionaryEntry { lemma: "zautrakati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zauvažati" => &[
        VerbDictionaryEntry { lemma: "zauvažati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zauvažiti" => &[
        VerbDictionaryEntry { lemma: "zauvažiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zavaliti" => &[
        VerbDictionaryEntry { lemma: "zavaliti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaviděti" => &[
        VerbDictionaryEntry { lemma: "zaviděti", addition: "(zavidi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zavladnųti" => &[
        VerbDictionaryEntry { lemma: "zavladnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zavladyvati" => &[
        VerbDictionaryEntry { lemma: "zavladyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zavojevati" => &[
        VerbDictionaryEntry { lemma: "zavojevati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zavojevyvati" => &[
        VerbDictionaryEntry { lemma: "zavojevyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zavraćati" => &[
        VerbDictionaryEntry { lemma: "zavraćati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zavråtiti" => &[
        VerbDictionaryEntry { lemma: "zavråtiti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zavyti" => &[
        VerbDictionaryEntry { lemma: "zavyti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zavęzati" => &[
        VerbDictionaryEntry { lemma: "zavęzati", addition: "(zavęže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zavęzyvati" => &[
        VerbDictionaryEntry { lemma: "zavęzyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zavědyvati" => &[
        VerbDictionaryEntry { lemma: "zavědyvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zavěćati" => &[
        VerbDictionaryEntry { lemma: "zavěćati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zavŕtěti" => &[
        VerbDictionaryEntry { lemma: "zavŕtěti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zavŕšati" => &[
        VerbDictionaryEntry { lemma: "zavŕšati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zavŕšiti" => &[
        VerbDictionaryEntry { lemma: "zavŕšiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "začarovati" => &[
        VerbDictionaryEntry { lemma: "začarovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "začinati" => &[
        VerbDictionaryEntry { lemma: "začinati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "začrveniti sę" => &[
        VerbDictionaryEntry { lemma: "začrveniti sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "začrvenjati sę" => &[
        VerbDictionaryEntry { lemma: "začrvenjati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "začuditi" => &[
        VerbDictionaryEntry { lemma: "začuditi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "začuditi sę" => &[
        VerbDictionaryEntry { lemma: "začuditi sę", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: true, intransitive: false, governs: None },
    ],
    "začęti" => &[
        VerbDictionaryEntry { lemma: "začęti", addition: "(začne)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zašifrovati" => &[
        VerbDictionaryEntry { lemma: "zašifrovati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zašiti" => &[
        VerbDictionaryEntry { lemma: "zašiti", addition: "(zašije)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaštopati" => &[
        VerbDictionaryEntry { lemma: "zaštopati", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaščititi" => &[
        VerbDictionaryEntry { lemma: "zaščititi", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaščićati" => &[
        VerbDictionaryEntry { lemma: "zaščićati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zaťmiti" => &[
        VerbDictionaryEntry { lemma: "zaťmiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zaťměvati" => &[
        VerbDictionaryEntry { lemma: "zaťměvati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zažartovati" => &[
        VerbDictionaryEntry { lemma: "zažartovati", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zažegti" => &[
        VerbDictionaryEntry { lemma: "zažegti", addition: "(zažže)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zažigati" => &[
        VerbDictionaryEntry { lemma: "zažigati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zdråvěti" => &[
        VerbDictionaryEntry { lemma: "zdråvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zirkati" => &[
        VerbDictionaryEntry { lemma: "zirkati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zirknųti" => &[
        VerbDictionaryEntry { lemma: "zirknųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "zlobiti" => &[
        VerbDictionaryEntry { lemma: "zlobiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zlobiti sę" => &[
        VerbDictionaryEntry { lemma: "zlobiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zloradovati sę" => &[
        VerbDictionaryEntry { lemma: "zloradovati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "zloupotrěbiti" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbiti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "zloupotrěbjati" => &[
        VerbDictionaryEntry { lemma: "zloupotrěbjati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zlåtiti" => &[
        VerbDictionaryEntry { lemma: "zlåtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "znamenovati" => &[
        VerbDictionaryEntry { lemma: "znamenovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "znati" => &[
        VerbDictionaryEntry { lemma: "znati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "značiti" => &[
        VerbDictionaryEntry { lemma: "značiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zrěti" => &[
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zri)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
        VerbDictionaryEntry { lemma: "zrěti", addition: "(zrěje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zvati" => &[
        VerbDictionaryEntry { lemma: "zvati", addition: "(zȯve)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "zvoniti" => &[
        VerbDictionaryEntry { lemma: "zvoniti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zvěrěti" => &[
        VerbDictionaryEntry { lemma: "zvěrěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zvųčati" => &[
        VerbDictionaryEntry { lemma: "zvųčati", addition: "(zvųči)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zębti" => &[
        VerbDictionaryEntry { lemma: "zębti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zějati" => &[
        VerbDictionaryEntry { lemma: "zějati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zěvati" => &[
        VerbDictionaryEntry { lemma: "zěvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "zěvnųti" => &[
        VerbDictionaryEntry { lemma: "zěvnųti", addition: "", transitive: false, imperfective: false, perfective: true, reflexive: false, intransitive: true, governs: None },
    ],
    "čaditi" => &[
        VerbDictionaryEntry { lemma: "čaditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "čarovati" => &[
        VerbDictionaryEntry { lemma: "čarovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "časovati" => &[
        VerbDictionaryEntry { lemma: "časovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čekati" => &[
        VerbDictionaryEntry { lemma: "čekati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "česati" => &[
        VerbDictionaryEntry { lemma: "česati", addition: "(češe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "činiti" => &[
        VerbDictionaryEntry { lemma: "činiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čisliti" => &[
        VerbDictionaryEntry { lemma: "čisliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čistiti" => &[
        VerbDictionaryEntry { lemma: "čistiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čitati" => &[
        VerbDictionaryEntry { lemma: "čitati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "črniti" => &[
        VerbDictionaryEntry { lemma: "črniti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "črněti" => &[
        VerbDictionaryEntry { lemma: "črněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "črpati" => &[
        VerbDictionaryEntry { lemma: "črpati", addition: "(črpe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "črstvěti" => &[
        VerbDictionaryEntry { lemma: "črstvěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "črtati" => &[
        VerbDictionaryEntry { lemma: "črtati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "črveněti" => &[
        VerbDictionaryEntry { lemma: "črveněti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "čtiti" => &[
        VerbDictionaryEntry { lemma: "čtiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čuditi" => &[
        VerbDictionaryEntry { lemma: "čuditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čuditi sę" => &[
        VerbDictionaryEntry { lemma: "čuditi sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "čuti" => &[
        VerbDictionaryEntry { lemma: "čuti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "čuvati" => &[
        VerbDictionaryEntry { lemma: "čuvati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "čuvati sę" => &[
        VerbDictionaryEntry { lemma: "čuvati sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "čučati" => &[
        VerbDictionaryEntry { lemma: "čučati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "čėstitati" => &[
        VerbDictionaryEntry { lemma: "čėstitati", addition: "", transitive: true, imperfective: true, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "čėstiti" => &[
        VerbDictionaryEntry { lemma: "čėstiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šalěti" => &[
        VerbDictionaryEntry { lemma: "šalěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "šantažovati" => &[
        VerbDictionaryEntry { lemma: "šantažovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šifrovati" => &[
        VerbDictionaryEntry { lemma: "šifrovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "širiti" => &[
        VerbDictionaryEntry { lemma: "širiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šiti" => &[
        VerbDictionaryEntry { lemma: "šiti", addition: "(šije)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "škoditi" => &[
        VerbDictionaryEntry { lemma: "škoditi", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "školiti" => &[
        VerbDictionaryEntry { lemma: "školiti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šlepati" => &[
        VerbDictionaryEntry { lemma: "šlepati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šlepnųti" => &[
        VerbDictionaryEntry { lemma: "šlepnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "šlifovati" => &[
        VerbDictionaryEntry { lemma: "šlifovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šokovati" => &[
        VerbDictionaryEntry { lemma: "šokovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "štopati" => &[
        VerbDictionaryEntry { lemma: "štopati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šuměti" => &[
        VerbDictionaryEntry { lemma: "šuměti", addition: "(šumi)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "ščekotati" => &[
        VerbDictionaryEntry { lemma: "ščekotati", addition: "(ščekoče)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ščeniti sę" => &[
        VerbDictionaryEntry { lemma: "ščeniti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "ščipati" => &[
        VerbDictionaryEntry { lemma: "ščipati", addition: "(ščipe)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ščipnųti" => &[
        VerbDictionaryEntry { lemma: "ščipnųti", addition: "", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "ščititi" => &[
        VerbDictionaryEntry { lemma: "ščititi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ščęditi" => &[
        VerbDictionaryEntry { lemma: "ščęditi", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "šėptati" => &[
        VerbDictionaryEntry { lemma: "šėptati", addition: "(šėpće)", transitive: true, imperfective: false, perfective: true, reflexive: false, intransitive: false, governs: None },
    ],
    "žaliti" => &[
        VerbDictionaryEntry { lemma: "žaliti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žaliti sę" => &[
        VerbDictionaryEntry { lemma: "žaliti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "žariti" => &[
        VerbDictionaryEntry { lemma: "žariti", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žartovati" => &[
        VerbDictionaryEntry { lemma: "žartovati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "žegti" => &[
        VerbDictionaryEntry { lemma: "žegti", addition: "(žže)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "želati" => &[
        VerbDictionaryEntry { lemma: "želati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "ženiti sę" => &[
        VerbDictionaryEntry { lemma: "ženiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "žiti" => &[
        VerbDictionaryEntry { lemma: "žiti", addition: "(žive)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "žrti" => &[
        VerbDictionaryEntry { lemma: "žrti", addition: "(žre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žrtvovati" => &[
        VerbDictionaryEntry { lemma: "žrtvovati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žrěbiti sę" => &[
        VerbDictionaryEntry { lemma: "žrěbiti sę", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: true, intransitive: false, governs: None },
    ],
    "žrěti" => &[
        VerbDictionaryEntry { lemma: "žrěti", addition: "(žre)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žuvati" => &[
        VerbDictionaryEntry { lemma: "žuvati", addition: "(žuje)", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "žužati" => &[
        VerbDictionaryEntry { lemma: "žužati", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
    "žędati" => &[
        VerbDictionaryEntry { lemma: "žędati", addition: "", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žęti" => &[
        VerbDictionaryEntry { lemma: "žęti", addition: "(žne)", transitive: true, imperfective: true, perfective: false, reflexive: false, intransitive: false, governs: None },
    ],
    "žȯltěti" => &[
        VerbDictionaryEntry { lemma: "žȯltěti", addition: "", transitive: false, imperfective: true, perfective: false, reflexive: false, intransitive: true, governs: None },
    ],
};

pub(crate) fn get_verbs(word: &str) -> Option<&'static [VerbDictionaryEntry]> {
    VERB_METADATA.get(word).copied()
}
