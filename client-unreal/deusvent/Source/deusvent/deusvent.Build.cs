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
			"EnhancedInput",
			"SQLiteCore",
			"SQLiteSupport",
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

		// Enable testing for non production builds
		if (Target.Configuration != UnrealTargetConfiguration.Shipping)
			PrivateDependencyModuleNames.AddRange(new[]
			{
				"AutomationController"
			});
	}
}
