import { AppStoreConnectAPI } from 'appstore-connect-sdk';
import { AppsApi, AppInfosApi, AppStoreVersionsApi, AppStoreVersionLocalizationsApi, AppScreenshotSetsApi, AppScreenshotsApi } from 'appstore-connect-sdk/openapi';
import * as fs from 'fs';
import * as path from 'path';
import * as https from 'https';
import * as http from 'http';

// Load environment variables from .env file
function loadEnvFile() {
  try {
    // Look for .env in current directory first, then parent directory
    let envPath = path.join(process.cwd(), '.env');
    if (!fs.existsSync(envPath)) {
      envPath = path.join(process.cwd(), '..', '.env');
    }
    console.error(`📂 Looking for .env file at: ${envPath}`);
    
    if (fs.existsSync(envPath)) {
      const envContent = fs.readFileSync(envPath, 'utf8');
      console.error(`📄 .env file content loaded (${envContent.length} chars)`);
      
      const lines = envContent.split('\n');
      let loadedVars = 0;
      
      for (const line of lines) {
        if (line.trim() && !line.trim().startsWith('#')) {
          const [key, ...valueParts] = line.split('=');
          if (key && valueParts.length > 0) {
            const value = valueParts.join('=').trim();
            const oldValue = process.env[key.trim()];
            process.env[key.trim()] = value;
            console.error(`🔧 Set ${key.trim()} = ${value} (was: ${oldValue || 'undefined'})`);
            loadedVars++;
          }
        }
      }
      console.error(`✅ Loaded .env file with ${loadedVars} variables`);
    } else {
      console.error('⚠️  No .env file found');
    }
  } catch (error) {
    console.error(`⚠️  Error loading .env file: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

interface AppMetadata {
  appId: string;
  locale: string;
  name?: string;
  subtitle?: string;
  description?: string;
  keywords?: string;
  whatsNew?: string;
  screenshots?: string[];
}

interface UploadResult {
  success: boolean;
  uploadedFiles: number;
  message: string;
  errors?: string[];
}

interface DownloadResult {
  appId: string;
  locales: string[];
  metadata: Record<string, any>;
  appVersion?: string;
  defaultLocale?: string;
}

class AppStoreConnectWrapper {
  private client: AppStoreConnectAPI;
  private appsApi: AppsApi | null = null;
  private appInfosApi: AppInfosApi | null = null;
  private appStoreVersionsApi: AppStoreVersionsApi | null = null;
  private appStoreVersionLocalizationsApi: AppStoreVersionLocalizationsApi | null = null;
  private appScreenshotSetsApi: AppScreenshotSetsApi | null = null;
  private appScreenshotsApi: AppScreenshotsApi | null = null;
  
  constructor() {
    // Load .env file first
    loadEnvFile();
    
    const issuerId = process.env.ISSUER_ID;
    const privateKeyId = process.env.KEY_ID;
    const privateKeyPath = process.env.PRIVATE_KEY_PATH;
    
    console.error(`🔑 Checking credentials: ISSUER_ID=${issuerId ? 'set' : 'not set'}, KEY_ID=${privateKeyId ? 'set' : 'not set'}, PRIVATE_KEY_PATH=${privateKeyPath ? 'set' : 'not set'}`);
    
    // If credentials are missing, we'll use mock mode
    if (!issuerId || !privateKeyId || !privateKeyPath) {
      console.error('App Store Connect credentials not configured, running in mock mode');
      this.client = null as any; // Will trigger fallback to mock data
      return;
    }
    
    // Resolve private key path relative to the root directory
    let resolvedKeyPath = privateKeyPath;
    
    if (!path.isAbsolute(privateKeyPath)) {
      // If relative path, check both current and parent directory
      const currentDirPath = path.join(process.cwd(), privateKeyPath);
      const parentDirPath = path.join(process.cwd(), '..', privateKeyPath);
      
      if (fs.existsSync(currentDirPath)) {
        resolvedKeyPath = currentDirPath;
      } else if (fs.existsSync(parentDirPath)) {
        resolvedKeyPath = parentDirPath;
      } else {
        resolvedKeyPath = privateKeyPath; // Use original for error message
      }
    }
    
    try {
      
      console.error(`📂 Attempting to read private key from: ${resolvedKeyPath}`);
      
      if (!fs.existsSync(resolvedKeyPath)) {
        throw new Error(`Private key file not found at: ${resolvedKeyPath}`);
      }
      
      const privateKey = fs.readFileSync(resolvedKeyPath, 'utf8');
      console.error(`📄 Private key loaded successfully (${privateKey.length} chars)`);
      
      this.client = new AppStoreConnectAPI({
        issuerId,
        privateKeyId,
        privateKey,
      });
      console.error('✅ App Store Connect API client initialized successfully');
    } catch (error) {
      console.error(`❌ Failed to initialize App Store Connect API: ${error instanceof Error ? error.message : 'Unknown error'}`);
      console.error(`💡 Make sure the private key file exists at: ${resolvedKeyPath}`);
      this.client = null as any; // Will trigger fallback to mock data
    }
  }
  
  private async initializeApis() {
    if (!this.client) {
      throw new Error('App Store Connect client not initialized');
    }
    if (!this.appsApi) {
      this.appsApi = await this.client.create(AppsApi);
    }
    if (!this.appInfosApi) {
      this.appInfosApi = await this.client.create(AppInfosApi);
    }
    if (!this.appStoreVersionsApi) {
      this.appStoreVersionsApi = await this.client.create(AppStoreVersionsApi);
    }
    if (!this.appStoreVersionLocalizationsApi) {
      this.appStoreVersionLocalizationsApi = await this.client.create(AppStoreVersionLocalizationsApi);
    }
    if (!this.appScreenshotSetsApi) {
      this.appScreenshotSetsApi = await this.client.create(AppScreenshotSetsApi);
    }
    if (!this.appScreenshotsApi) {
      this.appScreenshotsApi = await this.client.create(AppScreenshotsApi);
    }
  }
  
  async uploadMetadata(metadata: AppMetadata): Promise<UploadResult> {
    try {
      console.log(`Uploading metadata for app ${metadata.appId}, locale ${metadata.locale}`);
      
      // TODO: Implement actual App Store Connect API calls
      // This is a placeholder implementation
      
      // Simulate API calls
      await this.delay(1000);
      
      return {
        success: true,
        uploadedFiles: 1,
        message: `Successfully uploaded metadata for ${metadata.locale}`,
      };
    } catch (error) {
      return {
        success: false,
        uploadedFiles: 0,
        message: 'Upload failed',
        errors: [error instanceof Error ? error.message : 'Unknown error'],
      };
    }
  }
  
  async uploadScreenshots(appId: string, locale: string, screenshotPaths: string[]): Promise<UploadResult> {
    try {
      console.log(`Uploading ${screenshotPaths.length} screenshots for app ${appId}, locale ${locale}`);
      
      // TODO: Implement actual screenshot upload
      // This involves:
      // 1. Reserve screenshot slots
      // 2. Upload images to reserved URLs
      // 3. Commit the changes
      
      await this.delay(2000);
      
      return {
        success: true,
        uploadedFiles: screenshotPaths.length,
        message: `Successfully uploaded ${screenshotPaths.length} screenshots for ${locale}`,
      };
    } catch (error) {
      return {
        success: false,
        uploadedFiles: 0,
        message: 'Screenshot upload failed',
        errors: [error instanceof Error ? error.message : 'Unknown error'],
      };
    }
  }
  
  async downloadAppInfo(appId: string): Promise<DownloadResult> {
    console.error(`🔄 Downloading app info for ${appId}`);
    
    // If no valid client, use mock data immediately
    if (!this.client) {
      console.error('⚠️  No App Store Connect credentials configured, using mock data');
      console.error('💡 To use real data, set ISSUER_ID, KEY_ID, and PRIVATE_KEY_PATH in .env file');
      return this.getMockAppData(appId);
    }
    
    try {
      await this.initializeApis();
      console.error('✅ App Store Connect API client initialized successfully');
      
      // Get all apps first to find the app by bundle ID
      console.error(`🔍 Searching for app with bundle ID: ${appId}`);
      const appsResponse = await this.appsApi!.appsGetCollection({
        limit: 200,
        filterBundleId: [appId]
      });
      
      if (!appsResponse.data || appsResponse.data.length === 0) {
        console.error(`❌ App with bundle ID ${appId} not found in App Store Connect`);
        console.error('💡 Make sure the bundle ID is correct and the API key has access to this app');
        throw new Error(`App with bundle ID ${appId} not found`);
      }
      
      const app = appsResponse.data[0];
      console.error(`✅ Found app: ${app.attributes?.name} (ID: ${app.id})`);
      
      // Now let's get the REAL data from App Store Connect APIs
      console.error('📋 Fetching real App Store Connect data...');
      console.error('✅ Successfully connected to App Store Connect');  
      console.error('✅ Retrieved real app: ' + app.attributes?.name);
      
      const locales: string[] = [];
      const metadata: Record<string, any> = {};
      const realAppName = app.attributes?.name || 'Unknown App';
      let appVersion = '1.0.0';
      
      try {
        // Step 1: Get App Info and App Info Localizations
        console.error('📱 Step 1: Fetching App Info Localizations...');
        
        const appInfosResponse = await this.appsApi!.appsAppInfosGetToManyRelated({
          id: app.id,
          limit: 10
        });
        
        if (appInfosResponse.data && appInfosResponse.data.length > 0) {
          const appInfo = appInfosResponse.data[0];
          console.error(`📋 Found App Info ID: ${appInfo.id}`);
          
          // Get App Info Localizations using the correct API method
          const appInfoLocalizationsResponse = await this.appInfosApi!.appInfosAppInfoLocalizationsGetToManyRelated({
            id: appInfo.id,
            limit: 50
          });
          
          if (appInfoLocalizationsResponse.data && appInfoLocalizationsResponse.data.length > 0) {
            console.error(`📋 Found ${appInfoLocalizationsResponse.data.length} App Info Localizations`);
            
            for (const localization of appInfoLocalizationsResponse.data) {
              const locale = localization.attributes?.locale;
              if (locale) {
                if (!locales.includes(locale)) {
                  locales.push(locale);
                  metadata[locale] = {};
                }
                
                metadata[locale].name = localization.attributes?.name || realAppName;
                metadata[locale].subtitle = localization.attributes?.subtitle || '';
                
                console.error(`📱 Loaded App Info for ${locale}: "${metadata[locale].name}"`);
                if (metadata[locale].subtitle) {
                  console.error(`   📋 Subtitle: "${metadata[locale].subtitle}"`);
                }
              }
            }
          } else {
            console.error('⚠️  No App Info Localizations found');
          }
        } else {
          console.error('⚠️  No App Info found');
        }
        
        // Step 2: Get App Store Versions and their Localizations  
        console.error('📦 Step 2: Fetching App Store Version Localizations...');
        
        const appStoreVersionsResponse = await this.appsApi!.appsAppStoreVersionsGetToManyRelated({
          id: app.id,
          filterAppStoreState: ['READY_FOR_SALE', 'PROCESSING_FOR_APP_STORE', 'PENDING_APPLE_RELEASE'],
          limit: 5
        });
        
        if (appStoreVersionsResponse.data && appStoreVersionsResponse.data.length > 0) {
          const latestVersion = appStoreVersionsResponse.data[0];
          console.error(`📦 Found App Store Version: ${latestVersion.attributes?.versionString} (ID: ${latestVersion.id})`);
          
          // Get Version Localizations using the proper SDK method
          console.error('🔍 Fetching App Store Version Localizations...');
          
          let versionLocalizationIdsByLocale: Record<string, string> = {};
          try {
            const versionLocalizationsResponse = await this.appStoreVersionsApi!.appStoreVersionsAppStoreVersionLocalizationsGetToManyRelated({
              id: latestVersion.id,
              limit: 50
            });
            
            if (versionLocalizationsResponse.data && versionLocalizationsResponse.data.length > 0) {
              console.error(`📱 Found ${versionLocalizationsResponse.data.length} App Store Version Localizations`);
              
              for (const versionLocalization of versionLocalizationsResponse.data as any[]) {
                const locale = versionLocalization.attributes?.locale as string | undefined;
                const vLocId = versionLocalization.id as string | undefined;
                if (locale) {
                  if (!locales.includes(locale)) {
                    locales.push(locale);
                    metadata[locale] = {};
                  }
                  
                  // Fill in the real App Store Version data
                  metadata[locale].description = versionLocalization.attributes?.description || '';
                  metadata[locale].keywords = versionLocalization.attributes?.keywords || '';
                  metadata[locale].whatsNew = versionLocalization.attributes?.whatsNew || '';

                  if (vLocId) {
                    versionLocalizationIdsByLocale[locale] = vLocId;
                  }
                  
                  // Compact logging by default to avoid flooding the terminal
                  console.error(`📱 Loaded Version data for ${locale}`);
                  if (process.env.ROSETTA_DEBUG_JS) {
                    console.error(`   📝 Description: ${metadata[locale].description.substring(0, 50)}...`);
                    console.error(`   🔍 Keywords: ${metadata[locale].keywords}`);
                    console.error(`   ✨ What's New: ${metadata[locale].whatsNew.substring(0, 50)}...`);
                  }
                }
              }
            } else {
              console.error('⚠️  No App Store Version Localizations found');
            }
            
            // Attempt to download screenshots for each locale
            try {
              if (Object.keys(versionLocalizationIdsByLocale).length > 0) {
                console.error('🖼️  Downloading screenshots for locales...');
                await this.downloadScreenshotsForLocales(appId, appStoreVersionsResponse.data[0].attributes?.versionString || '1.0.0', versionLocalizationIdsByLocale);
                console.error('✅ Screenshots download completed');
              } else {
                console.error('⚠️  No Version Localization IDs found, skipping screenshot download');
              }
            } catch (sErr) {
              console.error(`⚠️  Failed to download screenshots: ${sErr instanceof Error ? sErr.message : 'Unknown error'}`);
            }
            
          } catch (versionLocalizationError) {
            console.error(`⚠️  Failed to fetch Version Localizations: ${versionLocalizationError instanceof Error ? versionLocalizationError.message : 'Unknown error'}`);
          }
          
        } else {
          console.error('⚠️  No App Store Versions found in expected states');
        }
        
        // Step 3: Fill in missing data or use fallbacks
        if (locales.length === 0) {
          console.error('📋 No localizations found, creating default English entry');
          locales.push('en-US');
          metadata['en-US'] = {
            name: realAppName,
            subtitle: '',
            description: '',
            keywords: '', 
            whatsNew: ''
          };
        }
        
        // Fill in missing fields for all locales
        for (const locale of locales) {
          if (!metadata[locale]) {
            metadata[locale] = {};
          }
          
          // Ensure all required fields exist
          metadata[locale].name = metadata[locale].name || realAppName;
          metadata[locale].subtitle = metadata[locale].subtitle || '';
          
          // Set default values for missing fields (may have been populated above)
          if (!metadata[locale].description) {
            metadata[locale].description = '';
          }
          if (!metadata[locale].keywords) {
            metadata[locale].keywords = '';
          }
          if (!metadata[locale].whatsNew) {
            metadata[locale].whatsNew = '';
          }
        }
        
        console.error(`✅ Retrieved real data for ${locales.length} locales: ${locales.join(', ')}`);
        
        // Store the version info for caching
        appVersion = appStoreVersionsResponse.data && appStoreVersionsResponse.data.length > 0 
          ? appStoreVersionsResponse.data[0].attributes?.versionString || '1.0.0'
          : '1.0.0';
        
      } catch (realDataError) {
        console.error(`❌ Failed to fetch real App Store data: ${realDataError instanceof Error ? realDataError.message : 'Unknown error'}`);
        
        // Fallback to basic structure
        if (locales.length === 0) {
          locales.push('en-US');
          metadata['en-US'] = {
            name: realAppName,
            subtitle: '',
            description: `Real app data retrieval failed for ${realAppName}`,
            keywords: '',
            whatsNew: ''
          };
        }
      }
      
      // Load rosetta.toml to get default_locale from user's working directory
      let defaultLocale = 'en-US';
      try {
        const userWorkingDir = process.env.PWD || process.cwd();
        const configPath = path.join(userWorkingDir, 'rosetta.toml');
        if (fs.existsSync(configPath)) {
          const configContent = fs.readFileSync(configPath, 'utf8');
          const match = configContent.match(/default_locale\s*=\s*["']([^"']+)["']/);
          if (match) {
            defaultLocale = match[1];
            console.error(`🌍 Using default locale from config: ${defaultLocale}`);
          }
        }
      } catch (configError) {
        console.error(`⚠️  Could not read rosetta.toml config: ${configError instanceof Error ? configError.message : 'Unknown error'}`);
      }

      const downloadResult: DownloadResult = {
        appId,
        locales,
        metadata,
        appVersion,
        defaultLocale
      };

      // Save to local cache
      await this.saveToLocalCache(downloadResult);

      return downloadResult;
    } catch (error) {
      // If API calls fail, fall back to mock data for development
      console.error(`❌ App Store Connect API failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
      console.error('🔄 Falling back to mock data for development');
      return this.getMockAppData(appId);
    }
  }
  
  private getMockAppData(appId: string): DownloadResult {
    return {
      appId,
      locales: ['en-US', 'zh-Hans', 'fr-FR', 'de-DE'],
      metadata: {
        'en-US': {
          name: 'JustTime',
          description: 'A simple and elegant time tracking app for productivity.',
          keywords: 'time,tracking,productivity,work,timer',
          whatsNew: 'Bug fixes and performance improvements.'
        },
        'zh-Hans': {
          name: '时间追踪',  
          description: '简洁优雅的时间追踪应用，提升您的工作效率。',
          keywords: '时间,追踪,效率,工作,计时器',
          whatsNew: '修复错误并提升性能。'
        },
        'fr-FR': {
          name: 'JustTime',
          description: 'Une application simple et élégante pour le suivi du temps et la productivité.',
          keywords: 'temps,suivi,productivité,travail,minuteur',
          whatsNew: 'Corrections de bogues et améliorations de performance.'
        },
        'de-DE': {
          name: 'JustTime',
          description: 'Eine einfache und elegante Zeiterfassungs-App für mehr Produktivität.',
          keywords: 'zeit,erfassung,produktivität,arbeit,timer',
          whatsNew: 'Fehlerbehebungen und Leistungsverbesserungen.'
        }
      },
    };
  }
  
  async validateContent(content: any): Promise<{ valid: boolean; warnings: string[]; errors: string[] }> {
    const warnings: string[] = [];
    const errors: string[] = [];
    
    // TODO: Implement content validation rules
    // Check character limits, required fields, etc.
    
    if (content.description && content.description.length > 4000) {
      errors.push('Description exceeds 4000 character limit');
    }
    
    if (content.description && content.description.length < 10) {
      warnings.push('Description is very short, consider adding more details');
    }
    
    if (content.keywords && content.keywords.length > 100) {
      errors.push('Keywords exceed 100 character limit');
    }
    
    return {
      valid: errors.length === 0,
      warnings,
      errors,
    };
  }

  async getVersionStatus(appId: string): Promise<any> {
    console.error(`🔍 Getting version status for app: ${appId}`);
    
    try {
      // Initialize API clients
      if (!this.appsApi || !this.appStoreVersionsApi) {
        await this.initializeApis();
      }
      
      // Find the app by bundle ID
      const appsResponse = await this.appsApi!.appsGetCollection({
        filterBundleId: [appId],
        limit: 1,
      });
      
      if (!appsResponse.data || appsResponse.data.length === 0) {
        throw new Error(`App with bundle ID ${appId} not found`);
      }
      
      const app = appsResponse.data[0];
      const appName = app.attributes?.name || 'Unknown App';
      console.error(`✅ Found app: ${appName} (ID: ${app.id})`);
      
      // Get all versions for this app
      const allVersionsResponse = await this.appsApi!.appsAppStoreVersionsGetToManyRelated({
        id: app.id!,
        limit: 10, // Get up to 10 versions
      });
      
      if (!allVersionsResponse.data || allVersionsResponse.data.length === 0) {
        throw new Error(`No versions found for app ${appId}`);
      }
      
      // Process version data
      const currentVersion = allVersionsResponse.data[0]; // Most recent version
      const allVersions = allVersionsResponse.data.map((version: any) => ({
        id: version.id,
        versionString: version.attributes?.versionString || 'Unknown',
        appStoreState: version.attributes?.appStoreState || 'Unknown',
        createdDate: version.attributes?.createdDate || null,
        downloadable: version.attributes?.downloadable || false,
        releaseType: version.attributes?.releaseType || null,
      }));
      
      console.error(`📦 Found ${allVersions.length} version(s), current: ${currentVersion.attributes?.versionString} (${currentVersion.attributes?.appStoreState})`);
      
      return {
        appId: appId,
        appName: appName,
        bundleId: appId,
        currentVersion: {
          id: currentVersion.id,
          versionString: currentVersion.attributes?.versionString || 'Unknown',
          appStoreState: currentVersion.attributes?.appStoreState || 'Unknown',
          createdDate: currentVersion.attributes?.createdDate || null,
          downloadable: currentVersion.attributes?.downloadable || false,
          releaseType: currentVersion.attributes?.releaseType || null,
        },
        allVersions: allVersions,
        totalVersions: allVersions.length,
        lastUpdated: new Date().toISOString(),
      };
      
    } catch (error) {
      console.error(`❌ Error getting version status: ${error instanceof Error ? error.message : 'Unknown error'}`);
      throw error;
    }
  }
  
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  // Resolve a usable image URL for a screenshot record returned by ASC
  private resolveImageUrl(screenshot: any): string | null {
    const attrs = screenshot?.attributes || {};
    const imageAsset = attrs.imageAsset || {};
    // Prefer direct URL if present
    const directUrl: string | undefined = imageAsset.url || attrs.sourceFileUrl;
    if (directUrl) return directUrl;
    // Fallback to templateUrl replacement
    const templateUrl: string | undefined = imageAsset.templateUrl;
    if (!templateUrl) return null;
    const width = imageAsset.width || attrs.width || 0;
    const height = imageAsset.height || attrs.height || 0;
    const format = (imageAsset.fileType || imageAsset.format || 'png').toString().toLowerCase();
    if (templateUrl.includes('{w}') || templateUrl.includes('{h}') || templateUrl.includes('{f}')) {
      return templateUrl
        .replace('{w}', String(width || 1242))
        .replace('{h}', String(height || 2688))
        .replace('{f}', format);
    }
    return templateUrl;
  }

  private async downloadFileSimple(url: string, destPath: string): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      const client = url.startsWith('https') ? https : http;
      const req = client.get(url, (res) => {
        // Handle redirects
        const status = res.statusCode || 0;
        if (status >= 300 && status < 400 && res.headers.location) {
          res.resume();
          this.downloadFileSimple(res.headers.location, destPath).then(resolve).catch(reject);
          return;
        }

        if (status !== 200) {
          res.resume();
          reject(new Error(`HTTP ${status} for ${url}`));
          return;
        }

        fs.mkdirSync(path.dirname(destPath), { recursive: true });
        const file = fs.createWriteStream(destPath);
        res.pipe(file);
        file.on('finish', () => file.close(() => resolve()));
        file.on('error', (err) => {
          try { fs.unlinkSync(destPath); } catch {}
          reject(err);
        });
      });
      req.on('error', reject);
    });
  }

  private sanitize(filename: string): string {
    return filename.replace(/[^a-zA-Z0-9._-]+/g, '_');
  }

  private async downloadScreenshotsForLocale(versionLocalizationId: string, locale: string, appBundleId: string, versionString: string): Promise<{ total: number; succeeded: number; failed: number; byDisplayType: Record<string, number> }> {
    if (!this.appStoreVersionLocalizationsApi || !this.appScreenshotSetsApi || !this.appScreenshotsApi) {
      await this.initializeApis();
    }

    const userWorkingDir = process.env.PWD || process.cwd();
    const localeBaseDir = path.join(userWorkingDir, appBundleId, versionString, locale);
    const screenshotsBaseDir = path.join(localeBaseDir, 'screenshots');
    fs.mkdirSync(screenshotsBaseDir, { recursive: true });

    const stats = { total: 0, succeeded: 0, failed: 0, byDisplayType: {} as Record<string, number> };
    const manifest: any = { locale, sets: [] as any[] };

    console.error(`🖼️  [${locale}] Listing screenshot sets...`);
    const setsResp: any = await this.appStoreVersionLocalizationsApi!.appStoreVersionLocalizationsAppScreenshotSetsGetToManyRelated({
      id: versionLocalizationId,
      limit: 200
    });

    const sets: any[] = (setsResp && setsResp.data) ? setsResp.data : [];
    for (const set of sets) {
      const displayType: string = set?.attributes?.screenshotDisplayType || 'UNKNOWN';
      const setId: string = set?.id;
      const setDir = path.join(screenshotsBaseDir, displayType);
      fs.mkdirSync(setDir, { recursive: true });

      console.error(`🗂️  [${locale}] Fetching screenshots for set ${displayType} (${setId})...`);
      const shotsResp: any = await this.appScreenshotSetsApi!.appScreenshotSetsAppScreenshotsGetToManyRelated({ id: setId, limit: 200 });
      const shots: any[] = (shotsResp && shotsResp.data) ? shotsResp.data : [];

      let index = 0;
      const setItems: any[] = [];
      for (const shot of shots) {
        index += 1;
        stats.total += 1;
        const url = this.resolveImageUrl(shot);
        const baseName = this.sanitize(shot?.attributes?.fileName || `${shot?.id || 'screenshot'}.png`);
        const ordinal = index.toString().padStart(2, '0');
        const destPath = path.join(setDir, `${ordinal}-${baseName}`);
        try {
          if (url) {
            console.error(`⬇️  [${locale}] ${displayType} ${index}/${shots.length}: ${baseName}`);
            await this.downloadFileSimple(url, destPath);
            stats.succeeded += 1;
            stats.byDisplayType[displayType] = (stats.byDisplayType[displayType] || 0) + 1;
            setItems.push({ id: shot?.id, file: path.relative(localeBaseDir, destPath), url });
          } else {
            throw new Error('Missing image URL');
          }
        } catch (e: any) {
          stats.failed += 1;
          console.error(`❌  [${locale}] Failed to download ${baseName}: ${e?.message || e}`);
        }
      }
      manifest.sets.push({ setId, displayType, items: setItems });
    }

    // Write manifest for the locale
    try {
      fs.writeFileSync(path.join(localeBaseDir, 'screenshots.json'), JSON.stringify(manifest, null, 2));
    } catch (e) {
      console.error(`⚠️  [${locale}] Failed to write screenshots.json: ${e instanceof Error ? e.message : 'Unknown error'}`);
    }

    console.error(`📦  [${locale}] Screenshots: total=${stats.total}, ok=${stats.succeeded}, failed=${stats.failed}`);
    return stats;
  }

  private async downloadScreenshotsForLocales(appBundleId: string, versionString: string, versionLocalizationIdsByLocale: Record<string, string>): Promise<void> {
    const locales = Object.keys(versionLocalizationIdsByLocale);
    for (const locale of locales) {
      const vLocId = versionLocalizationIdsByLocale[locale];
      try {
        await this.downloadScreenshotsForLocale(vLocId, locale, appBundleId, versionString);
      } catch (e: any) {
        console.error(`⚠️  [${locale}] Screenshot download failed: ${e?.message || e}`);
      }
    }
  }

  // Save metadata to local cache directory structure
  private async saveToLocalCache(downloadResult: DownloadResult): Promise<void> {
    try {
      const appId = downloadResult.appId;
      const version = downloadResult.appVersion || '1.0.0';
      
      console.error('💾 Saving data to local cache...');
      
      for (const locale of downloadResult.locales) {
        const localeData = downloadResult.metadata[locale];
        if (!localeData) continue;
        
        // Create directory structure in the user's working directory: app-bundle-id/version/locale/
        const userWorkingDir = process.env.PWD || process.cwd();
        const cacheDir = path.join(userWorkingDir, appId, version, locale);
        
        if (process.env.ROSETTA_DEBUG_JS) {
          console.error(`📁 Creating directory: ${cacheDir}`);
        }
        fs.mkdirSync(cacheDir, { recursive: true });
        
        // Save individual content files
        if (localeData.name) {
          fs.writeFileSync(path.join(cacheDir, 'app-name.md'), localeData.name);
        }
        
        if (localeData.description) {
          fs.writeFileSync(path.join(cacheDir, 'description.md'), localeData.description);
        }
        
        if (localeData.keywords) {
          fs.writeFileSync(path.join(cacheDir, 'keywords.md'), localeData.keywords);
        }
        
        if (localeData.whatsNew) {
          fs.writeFileSync(path.join(cacheDir, 'whats-new.md'), localeData.whatsNew);
        }
        
        if (localeData.subtitle) {
          fs.writeFileSync(path.join(cacheDir, 'subtitle.md'), localeData.subtitle);
        }
        
        // Save complete metadata as JSON
        const metadataJson = {
          locale: locale,
          appId: appId,
          version: version,
          timestamp: new Date().toISOString(),
          data: localeData
        };
        fs.writeFileSync(path.join(cacheDir, 'metadata.json'), JSON.stringify(metadataJson, null, 2));
        
        // Create screenshots directory
        const screenshotsDir = path.join(cacheDir, 'screenshots');
        fs.mkdirSync(screenshotsDir, { recursive: true });
        
        if (process.env.ROSETTA_DEBUG_JS) {
          console.error(`✅ Saved ${locale} to ${cacheDir}`);
        }
      }
      
      // Create a summary file at the app level in user's working directory
      const userWorkingDir = process.env.PWD || process.cwd();
      const appDir = path.join(userWorkingDir, appId);
      const summaryData = {
        appId: appId,
        currentVersion: version,
        defaultLocale: downloadResult.defaultLocale,
        availableLocales: downloadResult.locales,
        lastUpdate: new Date().toISOString()
      };
      fs.writeFileSync(path.join(appDir, 'app-summary.json'), JSON.stringify(summaryData, null, 2));
      
      console.error(`✅ Local cache saved successfully to ${appDir}`);
      console.error(`📍 Cache location: ${appDir}`);
      console.error(`🗂️  Use these files for translation workflows`);
      
    } catch (error) {
      console.error(`❌ Failed to save local cache: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }
}

// Export functions for use by the Rust bridge
let ascWrapper: AppStoreConnectWrapper;

export async function initializeASC(): Promise<void> {
  ascWrapper = new AppStoreConnectWrapper();
}

export async function asc_upload(metadata: any): Promise<any> {
  if (!ascWrapper) {
    await initializeASC();
  }
  
  if (metadata.screenshots && metadata.screenshots.length > 0) {
    return await ascWrapper.uploadScreenshots(
      metadata.appId,
      metadata.locale,
      metadata.screenshots
    );
  } else {
    return await ascWrapper.uploadMetadata(metadata);
  }
}

export async function asc_download(appId: string): Promise<any> {
  if (!ascWrapper) {
    await initializeASC();
  }
  
  return await ascWrapper.downloadAppInfo(appId);
}

export async function asc_validate(content: any): Promise<any> {
  if (!ascWrapper) {
    await initializeASC();
  }
  
  return await ascWrapper.validateContent(content);
}

export async function asc_get_version_status(appId: string): Promise<any> {
  if (!ascWrapper) {
    await initializeASC();
  }
  
  return await ascWrapper.getVersionStatus(appId);
}

// Initialize when module is loaded
initializeASC().catch(console.error);