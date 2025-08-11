import OpenAI from 'openai';

interface TranslationRequest {
  text: string;
  sourceLocale: string;
  targetLocale: string;
  context?: string;
  field?: string;
}

interface TranslationResult {
  translatedText: string;
  confidence: number;
  warnings?: string[];
}

interface BatchTranslationRequest {
  metadata: Record<string, any>;
  sourceLocale: string;
  targetLocales: string[];
  context?: string;
}

interface BatchTranslationResult {
  translations: Record<string, Record<string, any>>;
  totalCost: number;
  tokensUsed: {
    input: number;
    output: number;
  };
}

class OpenAIService {
  private client: OpenAI | null = null;
  private model: string = 'gpt-4o-mini';
  
  constructor() {
    const apiKey = process.env.OPENAI_API_KEY;
    
    if (!apiKey) {
      console.error('OpenAI API key not configured, translation will use mock mode');
      return;
    }
    
    try {
      this.client = new OpenAI({
        apiKey: apiKey,
      });
      console.error('OpenAI client initialized');
    } catch (error) {
      console.error(`Failed to initialize OpenAI client: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }
  
  private getLocaleDisplayName(locale: string): string {
    const localeMap: Record<string, string> = {
      'en-US': 'English',
      'zh-Hans': 'Simplified Chinese',
      'zh-Hant': 'Traditional Chinese',
      'fr-FR': 'French',
      'de-DE': 'German',
      'ja-JP': 'Japanese',
      'ko-KR': 'Korean',
      'es-ES': 'Spanish',
      'pt-BR': 'Portuguese (Brazil)',
      'it-IT': 'Italian',
      'ru-RU': 'Russian',
      'ar-SA': 'Arabic'
    };
    
    return localeMap[locale] || locale;
  }
  
  private buildTranslationPrompt(request: TranslationRequest): string {
    const sourceLanguage = this.getLocaleDisplayName(request.sourceLocale);
    const targetLanguage = this.getLocaleDisplayName(request.targetLocale);
    
    let prompt = `You are a professional app store translator. Translate the following ${sourceLanguage} text to ${targetLanguage}.

Important guidelines:
- This is for an App Store listing, so keep it engaging and professional
- Maintain the tone and style appropriate for mobile app marketing
- Keep character limits in mind (app names should be short, descriptions can be longer)
- Use natural, native-sounding language for the target locale
- Preserve any technical terms or brand names when appropriate

`;
    
    if (request.field) {
      const fieldGuidelines: Record<string, string> = {
        'name': '- Keep it short and memorable\n- Consider cultural preferences for app naming\n',
        'description': '- Be compelling and informative\n- Highlight key features and benefits\n',
        'keywords': '- Translate concepts, not just words\n- Use terms people actually search for\n',
        'whatsNew': '- Keep it concise and clear\n- Focus on user benefits\n'
      };
      
      if (fieldGuidelines[request.field]) {
        prompt += `Field-specific guidance for "${request.field}":\n${fieldGuidelines[request.field]}\n`;
      }
    }
    
    if (request.context) {
      prompt += `Additional context: ${request.context}\n\n`;
    }
    
    prompt += `Text to translate:\n"${request.text}"

Respond with ONLY the translated text, no explanations or additional commentary.`;
    
    return prompt;
  }
  
  async translateText(request: TranslationRequest): Promise<TranslationResult> {
    if (!this.client) {
      // Return mock translation for development
      return this.getMockTranslation(request);
    }
    
    try {
      const prompt = this.buildTranslationPrompt(request);
      
      const completion = await this.client.chat.completions.create({
        model: this.model,
        messages: [
          {
            role: 'user',
            content: prompt
          }
        ],
        temperature: 0.3, // Lower temperature for more consistent translations
        max_tokens: 1000,
      });
      
      const translatedText = completion.choices[0]?.message?.content?.trim();
      
      if (!translatedText) {
        throw new Error('No translation received from OpenAI');
      }
      
      return {
        translatedText,
        confidence: 0.95, // We could implement confidence scoring later
      };
    } catch (error) {
      console.error(`Translation failed for ${request.sourceLocale} -> ${request.targetLocale}: ${error instanceof Error ? error.message : 'Unknown error'}`);
      // Fall back to mock translation on error
      return this.getMockTranslation(request);
    }
  }
  
  async translateMetadata(request: BatchTranslationRequest): Promise<BatchTranslationResult> {
    if (!this.client) {
      return this.getMockBatchTranslation(request);
    }
    
    const translations: Record<string, Record<string, any>> = {};
    let totalInputTokens = 0;
    let totalOutputTokens = 0;
    
    for (const targetLocale of request.targetLocales) {
      if (targetLocale === request.sourceLocale) {
        // Skip translating to the same locale
        translations[targetLocale] = { ...request.metadata };
        continue;
      }
      
      translations[targetLocale] = {};
      
      // Translate each field in the metadata
      for (const [field, value] of Object.entries(request.metadata)) {
        if (typeof value !== 'string' || !value.trim()) {
          translations[targetLocale][field] = value;
          continue;
        }
        
        try {
          const translationResult = await this.translateText({
            text: value,
            sourceLocale: request.sourceLocale,
            targetLocale: targetLocale,
            context: request.context,
            field: field
          });
          
          translations[targetLocale][field] = translationResult.translatedText;
          
          // Estimate token usage (rough approximation)
          totalInputTokens += Math.ceil(value.length / 4);
          totalOutputTokens += Math.ceil(translationResult.translatedText.length / 4);
        } catch (error) {
          console.error(`Failed to translate ${field} to ${targetLocale}: ${error instanceof Error ? error.message : 'Unknown error'}`);
          translations[targetLocale][field] = value; // Keep original on error
        }
        
        // Small delay to avoid rate limits
        await this.delay(100);
      }
    }
    
    // Estimate cost (approximate pricing for gpt-4o-mini)
    const inputCost = (totalInputTokens / 1000) * 0.00015; // $0.150 per 1K tokens
    const outputCost = (totalOutputTokens / 1000) * 0.0006; // $0.600 per 1K tokens
    const totalCost = inputCost + outputCost;
    
    return {
      translations,
      totalCost,
      tokensUsed: {
        input: totalInputTokens,
        output: totalOutputTokens
      }
    };
  }
  
  private getMockTranslation(request: TranslationRequest): TranslationResult {
    // Simple mock translations for development
    const mockTranslations: Record<string, Record<string, string>> = {
      'zh-Hans': {
        'JustTime': '时间追踪',
        'A simple and elegant time tracking app for productivity.': '简洁优雅的时间追踪应用，提升您的工作效率。',
        'time,tracking,productivity,work,timer': '时间,追踪,效率,工作,计时器',
        'Bug fixes and performance improvements.': '修复错误并提升性能。'
      },
      'fr-FR': {
        'JustTime': 'JustTime',
        'A simple and elegant time tracking app for productivity.': 'Une application simple et élégante pour le suivi du temps et la productivité.',
        'time,tracking,productivity,work,timer': 'temps,suivi,productivité,travail,minuteur',
        'Bug fixes and performance improvements.': 'Corrections de bogues et améliorations de performance.'
      },
      'de-DE': {
        'JustTime': 'JustTime',
        'A simple and elegant time tracking app for productivity.': 'Eine einfache und elegante Zeiterfassungs-App für mehr Produktivität.',
        'time,tracking,productivity,work,timer': 'zeit,erfassung,produktivität,arbeit,timer',
        'Bug fixes and performance improvements.': 'Fehlerbehebungen und Leistungsverbesserungen.'
      }
    };
    
    const targetTranslations = mockTranslations[request.targetLocale];
    const translatedText = targetTranslations?.[request.text] || `[MOCK] ${request.text}`;
    
    return {
      translatedText,
      confidence: 0.8,
      warnings: ['Using mock translation (OpenAI not configured)']
    };
  }
  
  private getMockBatchTranslation(request: BatchTranslationRequest): BatchTranslationResult {
    const translations: Record<string, Record<string, any>> = {};
    
    for (const targetLocale of request.targetLocales) {
      if (targetLocale === request.sourceLocale) {
        translations[targetLocale] = { ...request.metadata };
        continue;
      }
      
      translations[targetLocale] = {};
      
      for (const [field, value] of Object.entries(request.metadata)) {
        if (typeof value === 'string') {
          const mockResult = this.getMockTranslation({
            text: value,
            sourceLocale: request.sourceLocale,
            targetLocale: targetLocale,
            field: field
          });
          translations[targetLocale][field] = mockResult.translatedText;
        } else {
          translations[targetLocale][field] = value;
        }
      }
    }
    
    return {
      translations,
      totalCost: 0.05, // Mock cost
      tokensUsed: {
        input: 500,
        output: 600
      }
    };
  }
  
  async estimateCost(request: BatchTranslationRequest): Promise<{ estimatedCost: number; tokenEstimate: number }> {
    let totalTokens = 0;
    
    for (const targetLocale of request.targetLocales) {
      if (targetLocale === request.sourceLocale) continue;
      
      for (const value of Object.values(request.metadata)) {
        if (typeof value === 'string') {
          // Rough token estimation: ~4 characters per token
          const inputTokens = Math.ceil(value.length / 4);
          const outputTokens = Math.ceil(value.length / 4); // Assume similar length output
          totalTokens += inputTokens + outputTokens;
        }
      }
    }
    
    // Estimate cost for gpt-4o-mini
    const inputCost = (totalTokens * 0.5 / 1000) * 0.00015;
    const outputCost = (totalTokens * 0.5 / 1000) * 0.0006;
    const estimatedCost = inputCost + outputCost;
    
    return {
      estimatedCost,
      tokenEstimate: totalTokens
    };
  }
  
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

// Export for use by the bridge
let openaiService: OpenAIService;

export async function initializeOpenAI(): Promise<void> {
  openaiService = new OpenAIService();
}

export async function ai_translate(request: any): Promise<any> {
  if (!openaiService) {
    await initializeOpenAI();
  }
  
  return await openaiService.translateMetadata(request);
}

export async function ai_estimate_cost(request: any): Promise<any> {
  if (!openaiService) {
    await initializeOpenAI();
  }
  
  return await openaiService.estimateCost(request);
}

// Initialize when module is loaded
initializeOpenAI().catch(console.error);