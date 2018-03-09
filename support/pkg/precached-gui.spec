Name:    precached-gui
Version: 0.1.0
Release: 0%{?dist}
Summary: precached-gui - A GUI for precached
URL:     https://x3n0m0rph59.github.io/precached/
License: GPLv3+

# Source0: https://github.com/X3n0m0rph59/precached-gui.git
Source0: https://github.com/X3n0m0rph59/%{name}/archive/master.tar.gz

BuildRoot: %{_tmppath}/%{name}-build

BuildRequires: gtk+-devel
BuildRequires: glib2-devel
BuildRequires: atk-devel
BuildRequires: cairo-devel
BuildRequires: gdk-pixbuf2-devel
BuildRequires: pango-devel
BuildRequires: systemd
BuildRequires: dbus-devel
BuildRequires: zeromq-devel
BuildRequires: cargo

Requires: gtk+ gdk-pixbuf2 dbus zeromq

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
install -Dp -m 0755 %{_builddir}/%{name}-%{version}/target/release/precached-gui %{buildroot}%{_sbindir}/precached-gui

%files
%license LICENSE
%{_sbindir}/precached-gui
%{_datarootdir}/metainfo/org.precache.precached-gui.appdata.xml

%changelog
