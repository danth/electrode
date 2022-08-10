self:
{ pkgs, config, lib, ... }:

with lib;

let
  package =
    # https://github.com/danth/stylix integration
    if config.lib ? stylix
    then self.packages.${pkgs.system}.default.overrideAttrs (_: {
      postPatch = with config.lib.stylix.colors; ''
        sed -i src/style.css \
          -e 's/#202020/#${base00}/g' \
          -e 's/#454545/#${base01}/g' \
          -e 's/#e0e0e0/#${base05}/g'
      '';
    })
    # normal usage
    else self.packages.${pkgs.system}.default;

in {
  options.programs.electrode = {
    enable = mkOption {
      description = "Whether to install the Electrode status bar";
      type = types.bool;
      # As this module has to be installed separately we can assume
      # that the user wants to use it by default.
      default = true;
    };

    extended = mkOption {
      description = "Whether to enable extra statistics such as CPU and memory usage";
      type = types.bool;
      default = false;
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
        "${package}/bin/electrode"
        + optionalString config.programs.electrode.extended " --extended";
    };

    fonts.fonts = [ pkgs.font-awesome ];
  };
}
