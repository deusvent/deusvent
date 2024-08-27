// Copyright Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;

public class deusventTarget : TargetRules
{
	public deusventTarget(TargetInfo Target) : base(Target)
	{
		Type = TargetType.Game;
		DefaultBuildSettings = BuildSettingsVersion.V5;
		IncludeOrderVersion = EngineIncludeOrderVersion.Unreal5_4;
		ExtraModuleNames.Add("deusvent");

		// Exceptions are needed for Rust C++ wrappers
		bForceEnableExceptions = true;
		bOverrideBuildEnvironment = true;
	}
}