self:
{ pkgs, config, lib, ... }:

with lib;

{
  options.programs.electrode = {
    enable = mkOption {
      description = "Whether to install the Electrode status bar";
      type = types.bool;
      # As this module has to be installed separately we can assume
      # that the user wants to use it by default.
      default = true;
    };

    color = mkOption {
      description = "Color of the status bar text";
      type = types.str;
      default = "#000000";
    };
  };

  config = mkIf config.programs.electrode.enable {
    systemd.user.services.electrode = {
      description = "Electrode status bar";

      after = [ "graphical-session-pre.target" ];
      before = [ "graphical-session.target" ];
      partOf = [ "graphical-session.target" ];
      wantedBy = [ "graphical-session.target" ];

      serviceConfig.ExecStart =
        "${self.packages.${pkgs.system}.default}/bin/electrode"
        + " --color ${config.programs.electrode.color}";
    };

    fonts.fonts = [ pkgs.font-awesome ];
  };
}
