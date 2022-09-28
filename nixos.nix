self:
{ pkgs, config, lib, ... }:

with lib;

{
  options.programs.electrode = {
    enable = mkOption {
      description = "Whether to install the Electrode status bar.";
      type = types.bool;
      # As this module has to be installed separately we can assume
      # that the user wants to use it by default.
      default = true;
    };

    css = mkOption {
      description = "Custom CSS styles for Electrode.";
      type = with types; nullOr lines;
      default = null;
    };
  };

  config = mkIf config.programs.electrode.enable {
    systemd.user.services.electrode = {
      description = "Electrode status bar";

      after = [ "graphical-session-pre.target" ];
      before = [ "graphical-session.target" ];
      partOf = [ "graphical-session.target" ];
      wantedBy = [ "graphical-session.target" ];

      serviceConfig.ExecStart = "${self.packages.${pkgs.system}.default}/bin/electrode";

      environment = mkIf (config.programs.electrode.css != null) {
        XDG_CONFIG_DIRS = "${pkgs.writeTextFile {
          name = "electrode.css";
          destination = "/electrode/style.css";
          text = config.programs.electrode.css;
        }}";
      };
    };

    fonts.fonts = [ pkgs.font-awesome ];
  };
}
