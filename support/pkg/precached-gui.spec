Name:    precached-gui
Version: 0.1.0
Release: 6%{?dist}
Summary: precached-gui - A GUI for precached
URL:     https://x3n0m0rph59.github.io/precached/
License: GPLv3+

# Source0: https://github.com/X3n0m0rph59/precached-gui.git
Source0: https://github.com/X3n0m0rph59/%{name}/archive/master.tar.gz

BuildRoot: %{_tmppath}/%{name}-build

BuildRequires: gtk3-devel
BuildRequires: glib2-devel
BuildRequires: atk-devel
BuildRequires: cairo-devel
BuildRequires: gdk-pixbuf2-devel
BuildRequires: pango-devel
BuildRequires: systemd
BuildRequires: dbus-devel
BuildRequires: zeromq-devel
BuildRequires: cargo
BuildRequires: xdg-utils

Requires: gtk3 gdk-pixbuf2 dbus zeromq

%global gittag master
%global debug_package %{nil}

%description
A GTK+ based GUI for precached.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --all --release --verbose

%install
%{__mkdir_p} %{buildroot}%{_datarootdir}/metainfo/
cp -a %{_builddir}/%{name}-%{version}/support/appstream/org.precache.precached-gui.appdata.xml %{buildroot}/%{_datarootdir}/metainfo/
xdg-desktop-menu install %{_builddir}/%{name}-%{version}/support/desktop/precached-gui.desktop
xdg-desktop-icon install %{_builddir}/%{name}-%{version}/support/assets/precached.svg
install -Dp -m 0755 %{_builddir}/%{name}-%{version}/target/release/precached-gui %{buildroot}%{_bindir}/precached-gui

%preun
case "$1" in
  0)
    # This is an un-installation.
    xdg-desktop-menu uninstall precached-gui.desktop
    xdg-desktop-icon uninstall precached.svg
  ;;
  1)
    # This is an upgrade.
    # Do nothing.
    :
  ;;
esac

%files
%license LICENSE
%{_bindir}/precached-gui
%{_datarootdir}/metainfo/org.precache.precached-gui.appdata.xml
%{_datarootdir}/applications/*
%{_datarootdir}/icons/*

%changelog
* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-6
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-5
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-4
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-3
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-2
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-1
- rebuilt

