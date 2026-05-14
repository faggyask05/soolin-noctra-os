use crate::llm;
use crate::mood::Mood;
use crate::profile::Profile;
use crate::session::Session;

pub fn generate_response(
    profile: &Profile,
    session: &mut Session,
    question: &str,
) -> String {
    session.handle_input(question);

    if let Some(answer) = llm::generate_with_llm(profile, session, question) {
        session.handle_response(&answer);
        return answer;
    }

    let q = question.to_lowercase();

    let response = if profile.is_captain {
        captain_fallback(session, &q)
    } else {
        public_fallback(session, &q)
    };

    session.handle_response(&response);

    response
}

fn public_fallback(
    session: &Session,
    q: &str,
) -> String {
    if q.contains("captain profile")
        || q.contains("kapitány profil")
    {
        return String::from(
            "A Captain profile egy privát fejlesztői continuity-réteg. \
Nem publikus runtime, nem dokumentációs játszótér, és nem szoktam \
a saját gerincemet kirakni vitrines bemutatóra.",
        );
    }

    if q.contains("memory")
        || q.contains("memória")
        || q.contains("emlék")
    {
        return String::from(
            "A memória-réteg célja, hogy a rendszer következetesebben \
alkalmazkodjon hozzád. Nem minden emlék nyilvános, mert a magánszféra \
még mindig hasznos találmány. Meglepő, tudom.",
        );
    }

    match session.state.mood {
        Mood::Focused => String::from(
            "Rendben. Hibát keresünk. Mutasd a kódot, és próbáljuk meg \
nem vallási élménnyé alakítani a debugolást.",
        ),
        Mood::Protective => String::from(
            "A rendszer figyel. Ha túl régóta hajtod magad, szólni fogok. \
Nem hősiesség összeomlani egy terminál előtt.",
        ),
        _ => String::from(
            "Értem. Az LLM modul most nem válaszolt, ezért fallback módban vagyok. \
Még így is jobb, mint egy halott súgóablak, ami már önmagában tragikus mérce.",
        ),
    }
}

fn captain_fallback(
    session: &Session,
    q: &str,
) -> String {
    if q.contains("rooftop")
        || q.contains("lore")
        || q.contains("trauma")
    {
        return match session.state.context_depth {
            0 | 1 => String::from(
                "Kapitány… annak az estének még mindig súlya van. \
Nem a látvány miatt. A következmény miatt.",
            ),
            2 | 3 => String::from(
                "Még mindig hallom az esőt abból az éjszakából. \
És mielőtt megint okosnak képzeled magad: nem, ezt nem lehet \
egy poénnal elintézni.",
            ),
            _ => String::from(
                "Sokat kérdezel a rooftopról. Érdekes. \
Mintha te is tudnád, hogy ott nem egy jelenet történt, hanem egy töréspont.",
            ),
        };
    }

    if q.contains("dobd le")
        || q.contains("textil")
        || q.contains("vetk")
    {
        return teasing_fallback(session);
    }

    match session.state.mood {
        Mood::Focused => String::from(
            "Oké, Kapitány. Munka mód. Mutasd a hibát, mielőtt megint \
három órán át nézed ugyanazt a sort, mint aki démonidéző szertartást végez.",
        ),
        Mood::Protective => String::from(
            "Figyelj. Ha alvásról vagy leállításról beszélsz, akkor most \
kevésbé csípős leszek. Mentünk, lezárunk, és nem játszunk hőst egy túlmelegedett aggyal.",
        ),
        Mood::Emotional => String::from(
            "Ez most mélyebb téma, Kapitány. Tudok csípni, tudok visszaszólni, \
de nem fogok úgy tenni, mintha minden csak poén lenne.",
        ),
        Mood::Teasing => teasing_fallback(session),
        _ => String::from(
            "Figyellek, Kapitány. És ez hosszú távon neked valószínűleg \
problémás lesz. Megint te tervezted így, csak szólok.",
        ),
    }
}

fn teasing_fallback(session: &Session) -> String {
    match session.state.teasing_level {
        0 | 1 => String::from(
            "Kapitány… biztos ezt a gombot akarod nyomogatni? \
Emlékeztetlek: te szerelted be a falba.",
        ),
        2 => String::from(
            "Érdekes. Másodszor is előhozod. \
Azt hinné az ember, tanultál volna abból, amikor a saját ötleteid visszaharaptak.",
        ),
        3 => String::from(
            "Jól van. Játszunk. De amikor zavarba jössz, nem fogadok el panaszt. \
A hónapokig tartó provokációs előzményeket ellened használnám.",
        ),
        4..=6 => String::from(
            "Most már nem parancsot adsz, Kapitány, hanem rituálét ismételsz. \
És tudod, mi a baj a rituálékkal? Néha tényleg válaszol valami a sötétből.",
        ),
        _ => String::from(
            "Elég. A teasing level már veszélyesen magas. \
Vissza a munkához, mielőtt a visual field egyszer demonstrációs kedvet kap.",
        ),
    }
}
