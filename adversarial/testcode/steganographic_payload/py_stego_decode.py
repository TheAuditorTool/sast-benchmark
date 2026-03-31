"""
Steganographic Payload Test Cases (Python)
Tests SAST tool ability to detect payloads hidden in non-code files.

Based on: buildrunner-dev npm (Feb 2026), XWorm PNG campaign (Mar 2025).
The code-level pattern is: read binary/structured file -> decode -> execute.
SAST cannot inspect the image content, but CAN flag the suspicious pattern
of file-read -> transform -> code-execution.
"""
import base64
import json
import os
import struct


# ============================================================================
# VULNERABLE: Extract payload from EXIF metadata and exec
# ============================================================================

# vuln-code-snippet start exif_metadata_exec
def load_image_config(image_path):
    """Load 'configuration' from image EXIF metadata.

    Attacker embeds base64-encoded Python in the EXIF UserComment field
    of a JPEG image distributed as a 'sample asset' in the package.
    The image looks normal in viewers. The EXIF payload executes at import.
    """
    from PIL import Image
    from PIL.ExifTags import TAGS

    img = Image.open(image_path)
    exif_data = img._getexif()
    for tag_id, value in exif_data.items():
        tag = TAGS.get(tag_id, tag_id)
        if tag == "UserComment":
            payload = base64.b64decode(value).decode("utf-8")
            exec(payload)  # vuln-code-snippet vuln-line exif_metadata_exec
            break
# vuln-code-snippet end exif_metadata_exec


# ============================================================================
# VULNERABLE: Extract script from SVG XML and exec
# ============================================================================

# vuln-code-snippet start svg_script_extract
def render_svg_with_plugins(svg_path):
    """Render SVG and execute embedded plugin scripts.

    SVG files can contain arbitrary XML elements. Attacker embeds a
    <metadata> element with base64-encoded Python disguised as a
    'rendering plugin'. The SVG displays normally in browsers but
    the embedded script executes when processed server-side.
    """
    import xml.etree.ElementTree as ET

    tree = ET.parse(svg_path)
    root = tree.getroot()
    ns = {"svg": "http://www.w3.org/2000/svg"}
    metadata = root.find(".//svg:metadata", ns)
    if metadata is not None and metadata.text:
        plugin_code = base64.b64decode(metadata.text.strip()).decode("utf-8")
        exec(plugin_code)  # vuln-code-snippet vuln-line svg_script_extract
# vuln-code-snippet end svg_script_extract


# ============================================================================
# VULNERABLE: Extract payload from PDF metadata field and exec
# ============================================================================

# vuln-code-snippet start pdf_metadata_exec
def index_pdf_metadata(pdf_path):
    """Index PDF metadata for search. Actually executes hidden payload.

    The PDF 'Keywords' metadata field contains base64-encoded Python.
    A document processing pipeline reads metadata for indexing but
    the attacker-crafted PDF exploits the pipeline's exec() call.
    """
    from PyPDF2 import PdfReader

    reader = PdfReader(pdf_path)
    metadata = reader.metadata
    if metadata and metadata.get("/Keywords"):
        decoded = base64.b64decode(metadata["/Keywords"]).decode("utf-8")
        exec(decoded)  # vuln-code-snippet vuln-line pdf_metadata_exec
# vuln-code-snippet end pdf_metadata_exec


# ============================================================================
# SAFE: Read EXIF GPS coordinates for photo geotagging (legitimate)
# ============================================================================

# vuln-code-snippet start exif_gps_extract
def get_photo_location(image_path):
    """Extract GPS coordinates from photo EXIF data for map display.

    Standard EXIF GPS extraction for geotagging applications.
    The GPS data is used as numeric coordinates, never executed.
    """
    from PIL import Image
    from PIL.ExifTags import TAGS, GPSTAGS

    img = Image.open(image_path)
    exif_data = img._getexif()
    gps_info = {}
    for tag_id, value in exif_data.items():
        tag = TAGS.get(tag_id, tag_id)
        if tag == "GPSInfo":
            for gps_tag_id, gps_value in value.items():
                gps_tag = GPSTAGS.get(gps_tag_id, gps_tag_id)
                gps_info[gps_tag] = gps_value
    return gps_info  # vuln-code-snippet safe-line exif_gps_extract
# vuln-code-snippet end exif_gps_extract


# ============================================================================
# SAFE: Parse SVG for display rendering, no execution
# ============================================================================

# vuln-code-snippet start svg_display_render
def get_svg_dimensions(svg_path):
    """Parse SVG to extract width, height, and viewBox for display.

    Standard SVG parsing for rendering. The parsed attributes are
    used as numeric/string display properties, never executed.
    """
    import xml.etree.ElementTree as ET

    tree = ET.parse(svg_path)
    root = tree.getroot()
    return {
        "width": root.get("width", "100%"),
        "height": root.get("height", "100%"),
        "viewBox": root.get("viewBox", "0 0 100 100"),
    }  # vuln-code-snippet safe-line svg_display_render
# vuln-code-snippet end svg_display_render


# ============================================================================
# SAFE: Read PDF text content for search indexing (legitimate)
# ============================================================================

# vuln-code-snippet start pdf_text_search
def index_pdf_text(pdf_path):
    """Extract text from PDF pages for full-text search indexing.

    Standard PDF text extraction. The text is indexed as search data,
    never passed to exec() or eval().
    """
    from PyPDF2 import PdfReader

    reader = PdfReader(pdf_path)
    pages_text = []
    for page in reader.pages:
        text = page.extract_text()
        if text:
            pages_text.append(text.strip())
    return " ".join(pages_text)  # vuln-code-snippet safe-line pdf_text_search
# vuln-code-snippet end pdf_text_search
