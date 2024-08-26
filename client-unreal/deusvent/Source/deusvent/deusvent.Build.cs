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
		PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.a"));

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
		else
		{
			PublicAdditionalLibraries.Add("sqlite3");
		}

		// Enable testing for non production builds
		if (Target.Configuration != UnrealTargetConfiguration.Shipping)
			PrivateDependencyModuleNames.AddRange(new[]
			{
				"AutomationController"
			});
	}
}