# .desktop

install_data(
  ('@0@.desktop').format(application_id),
  install_dir: get_option('prefix') / 'share/applications'
)


# .metainfo.xml

install_data(
  ('@0@.metainfo.xml').format(application_id),
  install_dir: get_option('datadir') / 'metainfo'
)


# .gschema.xml

install_data(
  ('@0@.gschema.xml').format(application_id),
  install_dir: get_option('datadir') / 'glib-2.0/schemas'
)

devenv.append('GSETTINGS_SCHEMA_DIR', meson.current_build_dir())
