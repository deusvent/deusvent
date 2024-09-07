using System.IO;
using UnrealBuildTool;

public class deusvent : ModuleRules
{
	public deusvent(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		PublicDependencyModuleNames.AddRange(new[]
		{
			"Core",
			"CoreUObject",
			"Engine",
			"InputCore",
			"EnhancedInput"
		});
		PrivateDependencyModuleNames.AddRange(new[] { "CADKernel", "WebSockets" });

		// Logic lib
		CppStandard = CppStandardVersion.Cpp20;

		if (Target.Platform == UnrealTargetPlatform.IOS)
		{
			PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.arm64.ios.a"));
		}
		else if (Target.Platform == UnrealTargetPlatform.Mac)
		{
			PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.arm64.darwin.a"));
		}
		else if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.amd64.linux.a"));
		}
		else if (Target.Platform == UnrealTargetPlatform.Android)
		{
			PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.arm64.android.a"));
		}
		else
		{
			throw new System.Exception("Unsupported platform for liblogic: " + Target.Platform);
		}

		// We assume sqlite3 is available everywhere
		// TODO Paths looks very specific to my machine, let's check if those are the same on CI once we have it
		if (Target.Platform == UnrealTargetPlatform.IOS)
		{
			var SDKPath =
				Path.Combine(
					"/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/lib/");
			PublicAdditionalLibraries.Add(Path.Combine(SDKPath, "libsqlite3.tbd"));
		}
		else if (Target.Platform == UnrealTargetPlatform.Mac)
		{
			var SDKPath =
				Path.Combine(
					"/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/lib/");
			PublicAdditionalLibraries.Add(Path.Combine(SDKPath, "libsqlite3.tbd"));
		}
		else if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			// SQLiteCore is a default plugin, so I suppose linking public sqlite3.h header from there is fine
			PublicIncludePaths.Add(Path.Combine(EngineDirectory, "Plugins/Runtime/Database/SQLiteCore/Source/SQLiteCore/Public/sqlite"));
			PublicAdditionalLibraries.Add("/usr/lib/x86_64-linux-gnu/libsqlite3.so");
		}
		else
		{
			throw new System.Exception("Unsupported platform for sqlite3: " + Target.Platform);
		}

		// Enable testing for non production builds
		if (Target.Configuration != UnrealTargetConfiguration.Shipping)
			PrivateDependencyModuleNames.AddRange(new[]
			{
				"AutomationController"
			});
	}
}
