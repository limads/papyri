# About

`papyri` is small, focused data visualization library based on `cairo-rs`. Its primary purpose
is to offer seamless plotting capabilities to Rust-based GTK applications. It supports line, scatter, bar, interval, area and label "mappings",
which are graphical representations of quantitative data. Eventually the library will export a C API, that can be leveraged by GTK applications
written in C, or embedded at other programming environments. The generated plots can be exported to svg, png and eps,
therefore it is also suited as a stand-alone visualization library targeted at printed documents or web-pages.

The application exports a command-line tool, `papyri`, that is cargo-installable and can be used to validate and evaluate plots
from a JSON definition.

# Showcase

The plots below were generated with papyri.

# Design

`papyri` is based on a strong separation between plot definition and rendering. `papyri::model`
is responsible to build plot models. Those are loosely-typed, serializable
data structures that can be used to interface with high-level applications such as
the command-line tool that reads and renders from a plot JSON definition, or programming environments based on
dynamic languages. If you have a client-server architecture and the client application only
needs to build plot definitions (and is not concerned with rendering) you only need to
use this module. All structures there offer easy-to-use builder-like patterns to construct
plot definitions. The definition can be serialized to JSON to be built from the command-line
or a server application.

If the library is compiled with the features "gdk", "gdk-pixbuf" and "cairo-rs", then the `papyri::render`
module is exported as well. This can be used by a server or the application directly to actually
render the plots. Note that cairo and the glib framework are system dependencies for renderization.
You can easily render into a cairo surface if you are working on a GTK application (using the DrawingArea
widget) or you can export plots directly.

