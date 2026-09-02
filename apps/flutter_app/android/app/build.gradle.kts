plugins { id("com.android.application"); id("dev.flutter.flutter-gradle-plugin") }
android {
 namespace = "com.documentconverter"
 compileSdk = 36
 compileOptions { sourceCompatibility = JavaVersion.VERSION_17; targetCompatibility = JavaVersion.VERSION_17 }
 defaultConfig { applicationId = "com.documentconverter"; minSdk = 24; targetSdk = 36; versionCode = 1; versionName = "1.0" }
 buildTypes { release { signingConfig = signingConfigs.getByName("debug") } }
}
flutter { source = "../.." }
