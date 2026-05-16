desktop_file = '@0@.desktop'.format(application_id)
metainfo_file = '@0@.metainfo.xml'.format(application_id)


# .desktop

i18n.merge_file(
  input:       desktop_file + '.in',
  output:      desktop_file,
  type:        'desktop',
  po_dir:      meson.project_source_root() / 'po',
  install:     true,
  install_dir: get_option('prefix') / 'share/applications',
)


# .metainfo.xml

i18n.merge_file(
  input:       metainfo_file + '.in',
  output:      metainfo_file,
  type:        'xml',
  po_dir:      meson.project_source_root() / 'po',
  install:     true,
  install_dir: get_option('datadir') / 'metainfo',
)


# .gschema.xml

install_data(
  ('@0@.gschema.xml').format(application_id),
  install_dir: get_option('datadir') / 'glib-2.0/schemas'
)
