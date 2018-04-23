Name:    precached-gui
Version: 0.1.0
Release: 16%{?dist}
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

Requires: gtk3 glib2 atk cairo gdk-pixbuf2 pango dbus zeromq

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
%{__mkdir_p} %{buildroot}%{_datarootdir}/applications/
%{__mkdir_p} %{buildroot}%{_datarootdir}/icons/hicolor/scalable/apps/
cp -a %{_builddir}/%{name}-%{version}/support/appstream/org.precache.precached-gui.appdata.xml %{buildroot}/%{_datarootdir}/metainfo/
cp -a %{_builddir}/%{name}-%{version}/support/desktop/precached-gui.desktop %{buildroot}/%{_datarootdir}/applications/precached-gui.desktop
cp -a %{_builddir}/%{name}-%{version}/support/assets/precached.svg %{buildroot}/%{_datarootdir}/icons/hicolor/scalable/apps/precached-gui.svg
install -Dp -m 0755 %{_builddir}/%{name}-%{version}/target/release/precached-gui %{buildroot}%{_bindir}/precached-gui

%post
case "$1" in
  1)
    # This is an initial install.
    xdg-desktop-menu forceupdate    
  ;;
  2)
    # This is an upgrade.
    xdg-desktop-menu forceupdate    
  ;;
esac

%files
%license LICENSE
%{_bindir}/precached-gui
%{_datarootdir}/metainfo/org.precache.precached-gui.appdata.xml
%{_datarootdir}/applications/precached-gui.desktop
%{_datarootdir}/icons/hicolor/scalable/apps/precached-gui.svg

%changelog
* Mon Apr 23 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-16
- rebuilt

* Wed Mar 14 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-15
- rebuilt

* Sat Mar 10 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-14
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-13
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-12
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-11
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-10
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-9
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-8
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-7
- rebuilt

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

