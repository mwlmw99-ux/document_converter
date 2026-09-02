pluginManagement {
    val flutterSdkPath = System.getenv("FLUTTER_ROOT") ?: error("FLUTTER_ROOT is not set")
    includeBuild("$flutterSdkPath/packages/flutter_tools/gradle")
    repositories { google(); mavenCentral(); gradlePluginPortal() }
}
plugins { id("dev.flutter.flutter-plugin-loader") version "1.0.0" }
dependencyResolutionManagement { repositoriesMode.set(RepositoriesMode.PREFER_SETTINGS); repositories { google(); mavenCentral() } }
rootProject.name = "document_converter"
include(":app")
