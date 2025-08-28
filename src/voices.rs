use crate::types::StaticVoice;

/// Elevanlabs common voice IDs as constants
pub mod all_voices {
    use super::StaticVoice;

    // Pre-made voices from ElevenLabs
    pub static WILL: StaticVoice = StaticVoice {
        voice_id: "bIHbv24MWmeRgasZH58o",
        name: "Will",
        gender: "male",
    };

    pub static THOMAS: StaticVoice = StaticVoice {
        voice_id: "GBv7mTt0atIp3Br8iCZE",
        name: "Thomas",
        gender: "male",
    };

    pub static CHARLIE: StaticVoice = StaticVoice {
        voice_id: "IKne3meq5aSn9XLyUdCD",
        name: "Charlie",
        gender: "male",
    };

    pub static GEORGE: StaticVoice = StaticVoice {
        voice_id: "JBFqnCBsd6RMkjVDRZzb",
        name: "George",
        gender: "male",
    };

    pub static CALLUM: StaticVoice = StaticVoice {
        voice_id: "N2lVS1w4EtoT3dr4eOWO",
        name: "Callum",
        gender: "male",
    };

    pub static LIAM: StaticVoice = StaticVoice {
        voice_id: "TX3LPaxmHKxFdv7VOQHJ",
        name: "Liam",
        gender: "male",
    };

    pub static CHARLOTTE: StaticVoice = StaticVoice {
        voice_id: "XB0fDUnXU5powFXDhCwa",
        name: "Charlotte",
        gender: "female",
    };

    pub static ALICE: StaticVoice = StaticVoice {
        voice_id: "Xb7hH8MSUJpSbSDYk0k2",
        name: "Alice",
        gender: "female",
    };

    pub static MATILDA: StaticVoice = StaticVoice {
        voice_id: "XrExE9yKIg1WjnnlVkGX",
        name: "Matilda",
        gender: "female",
    };

    pub static RACHEL: StaticVoice = StaticVoice {
        voice_id: "21m00Tcm4TlvDq8ikWAM",
        name: "Rachel",
        gender: "female",
    };

    pub static DOMI: StaticVoice = StaticVoice {
        voice_id: "AZnzlk1XvdvUeBnXmlld",
        name: "Domi",
        gender: "female",
    };

    pub static BELLA: StaticVoice = StaticVoice {
        voice_id: "EXAVITQu4vr4xnSDxMaL",
        name: "Bella",
        gender: "female",
    };

    pub static ANTONI: StaticVoice = StaticVoice {
        voice_id: "ErXwobaYiN019PkySvjV",
        name: "Antoni",
        gender: "male",
    };

    pub static ELLI: StaticVoice = StaticVoice {
        voice_id: "MF3mGyEYCl7XYWbV9V6O",
        name: "Elli",
        gender: "female",
    };

    pub static JOSH: StaticVoice = StaticVoice {
        voice_id: "TxGEqnHWrfWFTfGW9XjX",
        name: "Josh",
        gender: "male",
    };

    pub static ARNOLD: StaticVoice = StaticVoice {
        voice_id: "VR6AewLTigWG4xSOukaG",
        name: "Arnold",
        gender: "male",
    };

    pub static ADAM: StaticVoice = StaticVoice {
        voice_id: "pNInz6obpgDQGcFmaJgB",
        name: "Adam",
        gender: "male",
    };

    pub static SAM: StaticVoice = StaticVoice {
        voice_id: "yoZ06aMxZJJ28mfd3POQ",
        name: "Sam",
        gender: "male",
    };

    pub static SERENA: StaticVoice = StaticVoice {
        voice_id: "pMsXgVXv3BLzUgSXRplE",
        name: "Serena",
        gender: "female",
    };

    pub static ROGER: StaticVoice = StaticVoice {
        voice_id: "CwhRBWXzGAHq8TQ4Fs17",
        name: "Roger",
        gender: "male",
    };

    pub static RIVER: StaticVoice = StaticVoice {
        voice_id: "SAz9YHcvj6GT2YYXdXww",
        name: "River",
        gender: "neutral",
    };

    pub static PAUL: StaticVoice = StaticVoice {
        voice_id: "5Q0t7uMcjvnagumLfvZi",
        name: "Paul",
        gender: "male",
    };

    pub static PATRICK: StaticVoice = StaticVoice {
        voice_id: "ODq5zmih8GrVes37Dizd",
        name: "Patrick",
        gender: "male",
    };

    pub static NICOLE: StaticVoice = StaticVoice {
        voice_id: "piTKgcLEGmPE4e6mEKli",
        name: "Nicole",
        gender: "female",
    };

    pub static MIMI: StaticVoice = StaticVoice {
        voice_id: "zrHiDhphv9ZnVXBqCLjz",
        name: "Mimi",
        gender: "female",
    };

    pub static MICHAEL: StaticVoice = StaticVoice {
        voice_id: "flq6f7yk4E4fJM5XTYuZ",
        name: "Michael",
        gender: "male",
    };

    pub static MARK: StaticVoice = StaticVoice {
        voice_id: "UgBBYS2sOqTuMpoF3BR0",
        name: "Mark",
        gender: "male",
    };

    pub static LILY: StaticVoice = StaticVoice {
        voice_id: "pFZP5JQG7iQjIQuC4Bku",
        name: "Lily",
        gender: "female",
    };

    pub static LAURA: StaticVoice = StaticVoice {
        voice_id: "FGY2WhTYpPnrIDTdsKH5",
        name: "Laura",
        gender: "female",
    };

    pub static JOSEPH: StaticVoice = StaticVoice {
        voice_id: "Zlb1dXrM653N07WRdFW3",
        name: "Joseph",
        gender: "male",
    };

    pub static JESSIE: StaticVoice = StaticVoice {
        voice_id: "t0jbNlBVZ17f02VDIeMI",
        name: "Jessie",
        gender: "male",
    };

    pub static JESSICA: StaticVoice = StaticVoice {
        voice_id: "cgSgspJ2msm6clMCkdW9",
        name: "Jessica",
        gender: "female",
    };

    pub static JEREMY: StaticVoice = StaticVoice {
        voice_id: "bVMeCyTHy58xNoL34h3p",
        name: "Jeremy",
        gender: "male",
    };

    pub static JAMES: StaticVoice = StaticVoice {
        voice_id: "ZQe5CZNOzWyzPSCn5a3c",
        name: "James",
        gender: "male",
    };

    pub static IVANA: StaticVoice = StaticVoice {
        voice_id: "4NejU5DwQjevnR6mh3mb",
        name: "Ivanna",
        gender: "female",
    };

    pub static HARRY: StaticVoice = StaticVoice {
        voice_id: "SOYHLrjzK2X1ezoPC6cr",
        name: "Harry",
        gender: "male",
    };

    pub static GRACE: StaticVoice = StaticVoice {
        voice_id: "oWAxZDx7w5VEj9dCyTzz",
        name: "Grace",
        gender: "female",
    };

    pub static GLINDA: StaticVoice = StaticVoice {
        voice_id: "z9fAnlkpzviPz146aGWa",
        name: "Glinda",
        gender: "female",
    };

    pub static GIOVANNI: StaticVoice = StaticVoice {
        voice_id: "zcAOhNBS3c14rBihAFp1",
        name: "Giovanni",
        gender: "male",
    };

    pub static GIGI: StaticVoice = StaticVoice {
        voice_id: "jBpfuIE2acCO8z3wKNLl",
        name: "Gigi",
        gender: "female",
    };

    pub static FREYA: StaticVoice = StaticVoice {
        voice_id: "jsCqWAovK2LkecY7zXl4",
        name: "Freya",
        gender: "female",
    };

    pub static FIN: StaticVoice = StaticVoice {
        voice_id: "D38z5RcWu1voky8WS1ja",
        name: "Fin",
        gender: "male",
    };

    pub static ETHAN: StaticVoice = StaticVoice {
        voice_id: "g5CIjZEefAph4nQFvHAz",
        name: "Ethan",
        gender: "male",
    };

    pub static ERIC: StaticVoice = StaticVoice {
        voice_id: "cjVigY5qzO86Huf0OWal",
        name: "Eric",
        gender: "male",
    };

    pub static EMILY: StaticVoice = StaticVoice {
        voice_id: "LcfcDJNUP1GQjkzn1xUU",
        name: "Emily",
        gender: "female",
    };

    pub static DREW: StaticVoice = StaticVoice {
        voice_id: "29vD33N1CtxCmqQRPOHJ",
        name: "Drew",
        gender: "male",
    };

    pub static DOROTHY: StaticVoice = StaticVoice {
        voice_id: "ThT5KcBeYPX3keUQqHPh",
        name: "Dorothy",
        gender: "female",
    };

    pub static DAVE: StaticVoice = StaticVoice {
        voice_id: "CYw3kZ02Hs0563khs1Fj",
        name: "Dave",
        gender: "male",
    };

    pub static DANIEL: StaticVoice = StaticVoice {
        voice_id: "onwK4e9ZLuTAKqWW03F9",
        name: "Daniel",
        gender: "male",
    };

    pub static CLYDE: StaticVoice = StaticVoice {
        voice_id: "2EiwWnXFnvU5JabPnv8n",
        name: "Clyde",
        gender: "male",
    };

    pub static CHRIS: StaticVoice = StaticVoice {
        voice_id: "iP95p4xoKVk53GoZ742B",
        name: "Chris",
        gender: "male",
    };

    pub static CASSIDY: StaticVoice = StaticVoice {
        voice_id: "56AoDkrOh6qfVPDXZ7Pt",
        name: "Cassidy",
        gender: "female",
    };

    pub static BRIAN: StaticVoice = StaticVoice {
        voice_id: "nPczCjzI2devNBz1zQrb",
        name: "Brian",
        gender: "male",
    };

    pub static BILL: StaticVoice = StaticVoice {
        voice_id: "pqHfZKP75CvOlQylNhV4",
        name: "Bill",
        gender: "male",
    };

    pub static ARIA: StaticVoice = StaticVoice {
        voice_id: "9BWtsMINqrJLrRacOk9x",
        name: "Aria",
        gender: "female",
    };

    /// Get all available pre-built voices as a vector
    pub fn all() -> Vec<&'static StaticVoice> {
        vec![
            &WILL, &THOMAS, &CHARLIE, &GEORGE, &CALLUM, &LIAM, &CHARLOTTE, &ALICE, &MATILDA,
            &RACHEL, &DOMI, &BELLA, &ANTONI, &ELLI, &JOSH, &ARNOLD, &ADAM, &SAM,
        ]
    }

    /// Get all male voices
    pub fn male() -> Vec<&'static StaticVoice> {
        all().into_iter().filter(|v| v.gender == "male").collect()
    }

    /// Get all female voices
    pub fn female() -> Vec<&'static StaticVoice> {
        all().into_iter().filter(|v| v.gender == "female").collect()
    }

    /// Find a voice by name (case-insensitive)
    pub fn find_by_name(name: &str) -> Option<&'static StaticVoice> {
        all()
            .into_iter()
            .find(|v| v.name.to_lowercase() == name.to_lowercase())
    }
}
