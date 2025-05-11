# Nexus RTI proxy

Quick and dirty proxy that sits between the Nexus RTI API used for the [live map](https://metro-rti.nexus.org.uk/MapEmbedded) and anything that just wants a KML file (e.g. OsmAnd).

Things you can request:

- `/metrolines.kml`
- `/metrostations.kml`
- `/warming.kml`
- `/alerts.kml`
- `/trainstatuses.kml`
- `/traindirections.kml`
