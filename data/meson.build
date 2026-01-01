apps_dir = join_paths(get_option('prefix'), 'share', 'applications')

if get_option('nightly')
  install_data(
    sources: 'studio.planetpeanut.Bobby.Nightly.desktop',
    install_dir: apps_dir,
    rename: 'studio.planetpeanut.Bobby.desktop'
  )
else
  install_data(
    sources: 'studio.planetpeanut.Bobby.desktop',
    install_dir: apps_dir
  )
endif

install_data(
  'studio.planetpeanut.Bobby.metainfo.xml',
  install_dir: join_paths(get_option('datadir'), 'metainfo')
)

install_data(
  'studio.planetpeanut.Bobby-mime.xml',
  install_dir: join_paths(get_option('datadir'), 'mime', 'packages')
)

install_data(
  'studio.planetpeanut.Bobby.gschema.xml',
  install_dir: join_paths(get_option('datadir'), 'glib-2.0', 'schemas')
)
