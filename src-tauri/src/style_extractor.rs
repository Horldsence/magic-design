use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// Constants for limiting extracted items
const MAX_COLORS: usize = 10;
const MAX_FONTS: usize = 10;
const MAX_FONT_SIZES: usize = 15;
const MAX_SPACING: usize = 15;

// Lazy static regex patterns for performance
static COLOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(#[0-9a-f]{3,8}|rgba?\([^)]+\)|hsla?\([^)]+\))").unwrap()
});

static SPACING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+(?:\.\d+)?(?:px|em|rem|%|vh|vw))").unwrap()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleGuide {
    pub url: String,
    pub colors: ColorScheme,
    pub typography: Typography,
    pub spacing: SpacingSystem,
    pub components: Vec<ComponentStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary_colors: Vec<String>,
    pub background_colors: Vec<String>,
    pub text_colors: Vec<String>,
    pub border_colors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_families: Vec<String>,
    pub font_sizes: Vec<String>,
    pub font_weights: Vec<String>,
    pub line_heights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    pub margins: Vec<String>,
    pub paddings: Vec<String>,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    pub name: String,
    pub styles: HashMap<String, String>,
}

pub async fn fetch_website(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let html = response.text().await?;
    Ok(html)
}

pub fn fetch_website_blocking(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(html)
}

pub fn extract_styles(html: &str, url: &str) -> Result<StyleGuide> {
    let document = Html::parse_document(html);
    
    let colors = extract_colors(&document);
    let typography = extract_typography(&document);
    let spacing = extract_spacing(&document);
    let components = extract_components(&document);
    
    Ok(StyleGuide {
        url: url.to_string(),
        colors,
        typography,
        spacing,
        components,
    })
}

fn extract_colors(document: &Html) -> ColorScheme {
    let mut primary_colors = HashSet::new();
    let mut background_colors = HashSet::new();
    let mut text_colors = HashSet::new();
    let mut border_colors = HashSet::new();
    
    // Extract inline styles
    let style_selector = Selector::parse("[style]").unwrap();
    for element in document.select(&style_selector) {
        if let Some(style) = element.value().attr("style") {
            extract_colors_from_style(
                style,
                &mut primary_colors,
                &mut background_colors,
                &mut text_colors,
                &mut border_colors,
            );
        }
    }
    
    // Extract from style tags
    let style_tag_selector = Selector::parse("style").unwrap();
    for element in document.select(&style_tag_selector) {
        let css_content = element.text().collect::<String>();
        extract_colors_from_css(
            &css_content,
            &mut primary_colors,
            &mut background_colors,
            &mut text_colors,
            &mut border_colors,
        );
    }
    
    ColorScheme {
        primary_colors: primary_colors.into_iter().take(MAX_COLORS).collect(),
        background_colors: background_colors.into_iter().take(MAX_COLORS).collect(),
        text_colors: text_colors.into_iter().take(MAX_COLORS).collect(),
        border_colors: border_colors.into_iter().take(MAX_COLORS).collect(),
    }
}

fn extract_colors_from_style(
    style: &str,
    primary: &mut HashSet<String>,
    background: &mut HashSet<String>,
    text: &mut HashSet<String>,
    border: &mut HashSet<String>,
) {
    for cap in COLOR_REGEX.captures_iter(style) {
        let color = cap[1].to_string();
        
        if style.contains("background") {
            background.insert(color.clone());
        }
        if style.contains("color") && !style.contains("background") {
            text.insert(color.clone());
        }
        if style.contains("border") {
            border.insert(color.clone());
        }
        
        primary.insert(color);
    }
}

fn extract_colors_from_css(
    css: &str,
    primary: &mut HashSet<String>,
    background: &mut HashSet<String>,
    text: &mut HashSet<String>,
    border: &mut HashSet<String>,
) {
    for line in css.lines() {
        for cap in COLOR_REGEX.captures_iter(line) {
            let color = cap[1].to_string();
            
            if line.contains("background") {
                background.insert(color.clone());
            }
            if line.contains("color") && !line.contains("background") {
                text.insert(color.clone());
            }
            if line.contains("border") {
                border.insert(color.clone());
            }
            
            primary.insert(color);
        }
    }
}

fn extract_typography(document: &Html) -> Typography {
    let mut font_families = HashSet::new();
    let mut font_sizes = HashSet::new();
    let mut font_weights = HashSet::new();
    let mut line_heights = HashSet::new();
    
    // Extract from style attributes
    let style_selector = Selector::parse("[style]").unwrap();
    for element in document.select(&style_selector) {
        if let Some(style) = element.value().attr("style") {
            extract_typography_from_style(
                style,
                &mut font_families,
                &mut font_sizes,
                &mut font_weights,
                &mut line_heights,
            );
        }
    }
    
    // Extract from style tags
    let style_tag_selector = Selector::parse("style").unwrap();
    for element in document.select(&style_tag_selector) {
        let css_content = element.text().collect::<String>();
        extract_typography_from_css(
            &css_content,
            &mut font_families,
            &mut font_sizes,
            &mut font_weights,
            &mut line_heights,
        );
    }
    
    Typography {
        font_families: font_families.into_iter().take(MAX_FONTS).collect(),
        font_sizes: font_sizes.into_iter().take(MAX_FONT_SIZES).collect(),
        font_weights: font_weights.into_iter().take(MAX_FONTS).collect(),
        line_heights: line_heights.into_iter().take(MAX_FONTS).collect(),
    }
}

fn extract_typography_from_style(
    style: &str,
    families: &mut HashSet<String>,
    sizes: &mut HashSet<String>,
    weights: &mut HashSet<String>,
    heights: &mut HashSet<String>,
) {
    // Font family
    if let Some(family) = extract_css_property(style, "font-family") {
        families.insert(family);
    }
    
    // Font size
    if let Some(size) = extract_css_property(style, "font-size") {
        sizes.insert(size);
    }
    
    // Font weight
    if let Some(weight) = extract_css_property(style, "font-weight") {
        weights.insert(weight);
    }
    
    // Line height
    if let Some(height) = extract_css_property(style, "line-height") {
        heights.insert(height);
    }
}

fn extract_typography_from_css(
    css: &str,
    families: &mut HashSet<String>,
    sizes: &mut HashSet<String>,
    weights: &mut HashSet<String>,
    heights: &mut HashSet<String>,
) {
    for line in css.lines() {
        if let Some(family) = extract_css_property(line, "font-family") {
            families.insert(family);
        }
        if let Some(size) = extract_css_property(line, "font-size") {
            sizes.insert(size);
        }
        if let Some(weight) = extract_css_property(line, "font-weight") {
            weights.insert(weight);
        }
        if let Some(height) = extract_css_property(line, "line-height") {
            heights.insert(height);
        }
    }
}

fn extract_spacing(document: &Html) -> SpacingSystem {
    let mut margins = HashSet::new();
    let mut paddings = HashSet::new();
    let mut gaps = HashSet::new();
    
    // Extract from style attributes
    let style_selector = Selector::parse("[style]").unwrap();
    for element in document.select(&style_selector) {
        if let Some(style) = element.value().attr("style") {
            extract_spacing_from_style(style, &mut margins, &mut paddings, &mut gaps);
        }
    }
    
    // Extract from style tags
    let style_tag_selector = Selector::parse("style").unwrap();
    for element in document.select(&style_tag_selector) {
        let css_content = element.text().collect::<String>();
        extract_spacing_from_css(&css_content, &mut margins, &mut paddings, &mut gaps);
    }
    
    SpacingSystem {
        margins: margins.into_iter().take(MAX_SPACING).collect(),
        paddings: paddings.into_iter().take(MAX_SPACING).collect(),
        gaps: gaps.into_iter().take(MAX_FONTS).collect(),
    }
}

fn extract_spacing_from_style(
    style: &str,
    margins: &mut HashSet<String>,
    paddings: &mut HashSet<String>,
    gaps: &mut HashSet<String>,
) {
    for line in style.split(';') {
        if line.contains("margin") {
            for cap in SPACING_REGEX.captures_iter(line) {
                margins.insert(cap[1].to_string());
            }
        }
        if line.contains("padding") {
            for cap in SPACING_REGEX.captures_iter(line) {
                paddings.insert(cap[1].to_string());
            }
        }
        if line.contains("gap") {
            for cap in SPACING_REGEX.captures_iter(line) {
                gaps.insert(cap[1].to_string());
            }
        }
    }
}

fn extract_spacing_from_css(
    css: &str,
    margins: &mut HashSet<String>,
    paddings: &mut HashSet<String>,
    gaps: &mut HashSet<String>,
) {
    for line in css.lines() {
        if line.contains("margin") {
            for cap in SPACING_REGEX.captures_iter(line) {
                margins.insert(cap[1].to_string());
            }
        }
        if line.contains("padding") {
            for cap in SPACING_REGEX.captures_iter(line) {
                paddings.insert(cap[1].to_string());
            }
        }
        if line.contains("gap") {
            for cap in SPACING_REGEX.captures_iter(line) {
                gaps.insert(cap[1].to_string());
            }
        }
    }
}

fn extract_components(document: &Html) -> Vec<ComponentStyle> {
    let mut components = Vec::new();
    
    // Extract common components
    let component_selectors = vec![
        ("button", "button"),
        ("link", "a"),
        ("heading", "h1, h2, h3, h4, h5, h6"),
        ("paragraph", "p"),
        ("input", "input"),
        ("form", "form"),
        ("nav", "nav"),
        ("header", "header"),
        ("footer", "footer"),
    ];
    
    for (name, selector_str) in component_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for (idx, element) in document.select(&selector).enumerate().take(3) {
                let mut styles = HashMap::new();
                
                if let Some(style_attr) = element.value().attr("style") {
                    for style_part in style_attr.split(';') {
                        if let Some((key, value)) = style_part.split_once(':') {
                            styles.insert(
                                key.trim().to_string(),
                                value.trim().to_string(),
                            );
                        }
                    }
                }
                
                if let Some(class) = element.value().attr("class") {
                    styles.insert("class".to_string(), class.to_string());
                }
                
                if !styles.is_empty() {
                    components.push(ComponentStyle {
                        name: format!("{}-{}", name, idx + 1),
                        styles,
                    });
                }
            }
        }
    }
    
    components
}

fn extract_css_property(style: &str, property: &str) -> Option<String> {
    let pattern = format!(r"{}:\s*([^;]+)", regex::escape(property));
    let regex = Regex::new(&pattern).ok()?;
    
    regex.captures(style)
        .map(|cap| cap[1].trim().to_string())
}

pub fn generate_markdown(style_guide: &StyleGuide) -> String {
    let mut markdown = String::new();
    
    markdown.push_str(&format!("# Style Guide for {}\n\n", style_guide.url));
    markdown.push_str(&format!("Generated on: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Color Scheme
    markdown.push_str("## Color Scheme\n\n");
    
    markdown.push_str("### Primary Colors\n");
    if style_guide.colors.primary_colors.is_empty() {
        markdown.push_str("- No primary colors detected\n");
    } else {
        for color in &style_guide.colors.primary_colors {
            markdown.push_str(&format!("- `{}`\n", color));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Background Colors\n");
    if style_guide.colors.background_colors.is_empty() {
        markdown.push_str("- No background colors detected\n");
    } else {
        for color in &style_guide.colors.background_colors {
            markdown.push_str(&format!("- `{}`\n", color));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Text Colors\n");
    if style_guide.colors.text_colors.is_empty() {
        markdown.push_str("- No text colors detected\n");
    } else {
        for color in &style_guide.colors.text_colors {
            markdown.push_str(&format!("- `{}`\n", color));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Border Colors\n");
    if style_guide.colors.border_colors.is_empty() {
        markdown.push_str("- No border colors detected\n");
    } else {
        for color in &style_guide.colors.border_colors {
            markdown.push_str(&format!("- `{}`\n", color));
        }
    }
    markdown.push('\n');
    
    // Typography
    markdown.push_str("## Typography\n\n");
    
    markdown.push_str("### Font Families\n");
    if style_guide.typography.font_families.is_empty() {
        markdown.push_str("- No font families detected\n");
    } else {
        for font in &style_guide.typography.font_families {
            markdown.push_str(&format!("- `{}`\n", font));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Font Sizes\n");
    if style_guide.typography.font_sizes.is_empty() {
        markdown.push_str("- No font sizes detected\n");
    } else {
        for size in &style_guide.typography.font_sizes {
            markdown.push_str(&format!("- `{}`\n", size));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Font Weights\n");
    if style_guide.typography.font_weights.is_empty() {
        markdown.push_str("- No font weights detected\n");
    } else {
        for weight in &style_guide.typography.font_weights {
            markdown.push_str(&format!("- `{}`\n", weight));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Line Heights\n");
    if style_guide.typography.line_heights.is_empty() {
        markdown.push_str("- No line heights detected\n");
    } else {
        for height in &style_guide.typography.line_heights {
            markdown.push_str(&format!("- `{}`\n", height));
        }
    }
    markdown.push('\n');
    
    // Spacing System
    markdown.push_str("## Spacing System\n\n");
    
    markdown.push_str("### Margins\n");
    if style_guide.spacing.margins.is_empty() {
        markdown.push_str("- No margins detected\n");
    } else {
        for margin in &style_guide.spacing.margins {
            markdown.push_str(&format!("- `{}`\n", margin));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Paddings\n");
    if style_guide.spacing.paddings.is_empty() {
        markdown.push_str("- No paddings detected\n");
    } else {
        for padding in &style_guide.spacing.paddings {
            markdown.push_str(&format!("- `{}`\n", padding));
        }
    }
    markdown.push('\n');
    
    markdown.push_str("### Gaps\n");
    if style_guide.spacing.gaps.is_empty() {
        markdown.push_str("- No gaps detected\n");
    } else {
        for gap in &style_guide.spacing.gaps {
            markdown.push_str(&format!("- `{}`\n", gap));
        }
    }
    markdown.push('\n');
    
    // Component Styles
    markdown.push_str("## Component Styles\n\n");
    if style_guide.components.is_empty() {
        markdown.push_str("No component styles detected\n");
    } else {
        for component in &style_guide.components {
            markdown.push_str(&format!("### {}\n", component.name));
            for (key, value) in &component.styles {
                markdown.push_str(&format!("- **{}**: `{}`\n", key, value));
            }
            markdown.push('\n');
        }
    }
    
    markdown
}
