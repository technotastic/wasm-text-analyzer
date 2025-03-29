use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::Serialize;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct TextAnalysis {
    text: String,
    word_count: usize,
    character_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
    #[serde(skip)]
    word_frequency: HashMap<String, usize>,
}

#[wasm_bindgen]
impl TextAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new(text: &str) -> TextAnalysis {
        let cleaned_text = text.trim();
        let word_count = count_words(cleaned_text);
        let character_count = count_characters(cleaned_text);
        let sentence_count = count_sentences(cleaned_text);
        let paragraph_count = count_paragraphs(cleaned_text);
        let word_frequency = calculate_word_frequency(cleaned_text);

        TextAnalysis {
            text: cleaned_text.to_string(),
            word_count,
            character_count,
            sentence_count,
            paragraph_count,
            word_frequency,
        }
    }

    pub fn word_count(&self) -> usize {
        self.word_count
    }

    pub fn character_count(&self) -> usize {
        self.character_count
    }

    pub fn sentence_count(&self) -> usize {
        self.sentence_count
    }

    pub fn paragraph_count(&self) -> usize {
        self.paragraph_count
    }

    pub fn calculate_readability(&self) -> f64 {
        // Simple Flesch-Kincaid Grade Level formula
        // 0.39 * (words/sentences) + 11.8 * (syllables/words) - 15.59
        if self.sentence_count == 0 || self.word_count == 0 {
            return 0.0;
        }

        let words_per_sentence = self.word_count as f64 / self.sentence_count as f64;
        let syllables = estimate_syllables(&self.text);
        let syllables_per_word = syllables as f64 / self.word_count as f64;

        0.39 * words_per_sentence + 11.8 * syllables_per_word - 15.59
    }

    pub fn get_top_keywords(&self, count: usize) -> JsValue {
        let mut word_vec: Vec<(String, usize)> = self.word_frequency.clone().into_iter().collect();
        word_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        let top_words: Vec<(String, usize)> = word_vec.into_iter()
            .take(count)
            .collect();
            
        serde_wasm_bindgen::to_value(&top_words).unwrap()
    }

    pub fn get_sentiment(&self) -> f64 {
        // Simplified sentiment analysis
        // In a real implementation, you would use a proper sentiment lexicon
        let positive_words = ["good", "great", "excellent", "happy", "positive", "wonderful", "best", "love"];
        let negative_words = ["bad", "awful", "terrible", "sad", "negative", "worst", "hate", "poor"];
        
        // Fixed: Create owned strings to avoid reference to temporary value
        let words: Vec<String> = self.text.split_whitespace()
            .map(|w| {
                let lowercase = w.to_lowercase();
                lowercase.trim_matches(|c: char| !c.is_alphabetic()).to_string()
            })
            .collect();
            
        let mut sentiment_score = 0.0;
        let mut word_count = 0;
        
        for word in &words {
            if positive_words.contains(&word.as_str()) {
                sentiment_score += 1.0;
                word_count += 1;
            } else if negative_words.contains(&word.as_str()) {
                sentiment_score -= 1.0;
                word_count += 1;
            }
        }
        
        if word_count > 0 {
            sentiment_score / word_count as f64
        } else {
            0.0
        }
    }
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_characters(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

fn count_sentences(text: &str) -> usize {
    text.split(|c| c == '.' || c == '!' || c == '?')
        .filter(|s| !s.trim().is_empty())
        .count()
}

fn count_paragraphs(text: &str) -> usize {
    text.split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .count()
}

fn calculate_word_frequency(text: &str) -> HashMap<String, usize> {
    let mut word_map = HashMap::new();
    
    for word in text.split_whitespace() {
        let lowercase = word.to_lowercase();
        let cleaned_word = lowercase.trim_matches(|c: char| !c.is_alphabetic()).to_string();
            
        if !cleaned_word.is_empty() && cleaned_word.len() > 2 {
            *word_map.entry(cleaned_word).or_insert(0) += 1;
        }
    }
    
    word_map
}

fn estimate_syllables(text: &str) -> usize {
    // Fixed: Create owned strings to avoid reference to temporary value
    let words: Vec<String> = text.split_whitespace()
        .map(|w| {
            let lowercase = w.to_lowercase();
            lowercase.trim_matches(|c: char| !c.is_alphabetic()).to_string()
        })
        .collect();
        
    let mut total_syllables = 0;
    
    for word in words {
        if word.is_empty() {
            continue;
        }
        
        // Simple heuristic for English syllable counting
        let mut syllable_count: usize = 0;
        let chars: Vec<char> = word.chars().collect();
        let mut prev_is_vowel = false;
        
        for (_i, &c) in chars.iter().enumerate() {
            let is_vowel = "aeiouy".contains(c);
            
            if is_vowel && !prev_is_vowel {
                syllable_count += 1;
            }
            
            prev_is_vowel = is_vowel;
        }
        
        // Handle silent e at the end
        if word.len() > 2 && word.ends_with('e') && !word.ends_with("le") {
            syllable_count = syllable_count.saturating_sub(1);
        }
        
        // Ensure at least one syllable per word
        if syllable_count == 0 {
            syllable_count = 1;
        }
        
        total_syllables += syllable_count;
    }
    
    total_syllables
}

#[wasm_bindgen]
pub fn analyze_text(text: &str) -> JsValue {
    let analysis = TextAnalysis::new(text);
    let result = serde_wasm_bindgen::to_value(&analysis).unwrap();
    result
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen(start)]
pub fn start() {
    // This function is called when the WebAssembly module is instantiated
    console_log!("Text Analysis Tool initialized");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/*macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}*/
