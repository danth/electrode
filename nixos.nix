self:
{ pkgs, config, lib, ... }:

with lib;

{
  options.programs.electrode.enable = mkOption {
    description = "Whether to install the Electrode status bar";
    type = types.bool;
    # As this module has to be installed separately we can assume
    # that the user wants to use it by default.
    default = true;
  };

  config = mkIf config.programs.electrode.enable {
    systemd.user.services.electrode = {
      description = "Electrode status bar";

      after = [ "graphical-session-pre.target" ];
      before = [ "graphical-session.target" ];
      partOf = [ "graphical-session.target" ];
      wantedBy = [ "graphical-session.target" ];

      serviceConfig.ExecStart = "${self.packages.${pkgs.system}.default}/bin/electrode";
    };

    fonts.fonts = [ pkgs.font-awesome ];
  };
}
