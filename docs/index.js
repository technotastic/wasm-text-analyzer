import init, { TextAnalysis } from '../pkg/text_analysis_tool.js';

async function run() {
    await init();
    
    const analyzeBtn = document.getElementById('analyze-btn');
    const textInput = document.getElementById('text-input');
    
    analyzeBtn.addEventListener('click', () => {
        const text = textInput.value;
        if (!text.trim()) {
            alert('Please enter some text to analyze');
            return;
        }
        
        analyzeText(text);
    });
    
    // Add sample text for testing
    textInput.value = "This is a sample text for analysis. It contains multiple sentences with various words. The purpose of this text is to demonstrate the capabilities of our Text Analysis Tool built with Rust and WebAssembly. This tool can analyze readability, extract keywords, and perform sentiment analysis on any text you provide.";
}

function analyzeText(text) {
    try {
        const analysis = new TextAnalysis(text);
        
        // Update basic metrics
        document.getElementById('word-count').textContent = analysis.word_count();
        document.getElementById('character-count').textContent = analysis.character_count();
        document.getElementById('sentence-count').textContent = analysis.sentence_count();
        document.getElementById('paragraph-count').textContent = analysis.paragraph_count();
        
        // Update readability
        const readabilityScore = analysis.calculate_readability();
        document.getElementById('fk-score').textContent = readabilityScore.toFixed(1);
        
        // Update readability gauge
        const gaugeFill = document.getElementById('gauge-fill');
        const normalizedScore = Math.min(Math.max(readabilityScore, 0), 18) / 18;
        gaugeFill.style.width = `${normalizedScore * 100}%`;
        
        // Interpret readability
        let interpretation = '';
        if (readabilityScore < 6) {
            interpretation = 'Very easy to read';
            gaugeFill.style.backgroundColor = '#2ecc71';
        } else if (readabilityScore < 10) {
            interpretation = 'Easy to read';
            gaugeFill.style.backgroundColor = '#3498db';
        } else if (readabilityScore < 14) {
            interpretation = 'Moderately difficult';
            gaugeFill.style.backgroundColor = '#f39c12';
        } else {
            interpretation = 'Difficult to read';
            gaugeFill.style.backgroundColor = '#e74c3c';
        }
        document.getElementById('readability-interpretation').textContent = interpretation;
        
        // Update keywords
        const keywords = analysis.get_top_keywords(10);
        const keywordsList = document.getElementById('keywords-list');
        keywordsList.innerHTML = '';
        
        keywords.forEach(([word, count]) => {
            const tag = document.createElement('span');
            tag.className = 'keyword-tag';
            tag.textContent = `${word} (${count})`;
            keywordsList.appendChild(tag);
        });
        
        // Update sentiment
        const sentiment = analysis.get_sentiment();
        const sentimentIndicator = document.getElementById('sentiment-indicator');
        const normalizedSentiment = (sentiment + 1) / 2; // Convert from [-1, 1] to [0, 1]
        sentimentIndicator.style.width = `${normalizedSentiment * 100}%`;
        
        let sentimentText = '';
        if (sentiment < -0.3) {
            sentimentText = 'Negative';
            sentimentIndicator.style.backgroundColor = '#e74c3c';
        } else if (sentiment > 0.3) {
            sentimentText = 'Positive';
            sentimentIndicator.style.backgroundColor = '#2ecc71';
        } else {
            sentimentText = 'Neutral';
            sentimentIndicator.style.backgroundColor = '#95a5a6';
        }
        document.getElementById('sentiment-value').textContent = sentimentText;
        
    } catch (error) {
        console.error('Analysis error:', error);
        alert('An error occurred during text analysis');
    }
}

// Initialize the application
run().catch(console.error);
