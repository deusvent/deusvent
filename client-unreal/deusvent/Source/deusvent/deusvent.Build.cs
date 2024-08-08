// Copyright Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;
using System.IO;

public class deusvent : ModuleRules
{
	public deusvent(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "InputCore", "EnhancedInput" });

		PrivateDependencyModuleNames.AddRange(new string[] { });

		// Logic lib
		CppStandard = CppStandardVersion.Cpp20;
		PublicAdditionalLibraries.Add(Path.Combine(ModuleDirectory, "../../ThirdParty/liblogic.a"));

	}
}
