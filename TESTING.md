# Testing Guide

## Unit Tests

The Rust backend can be tested using cargo test (requires system dependencies):

```bash
cd src-tauri
cargo test
```

## Manual Testing

Since the application requires system dependencies to build on Linux, here's how to test once built:

### Test Cases

1. **Valid URL Test**
   - Input: `https://example.com`
   - Expected: Successfully extracts styles and generates markdown

2. **HTTPS URL Test**
   - Input: `https://github.com`
   - Expected: Successfully fetches and parses styles

3. **Invalid URL Test**
   - Input: `not-a-url`
   - Expected: Shows error message

4. **Network Error Test**
   - Input: `https://invalid-domain-that-does-not-exist.com`
   - Expected: Shows appropriate error message

5. **Copy to Clipboard Test**
   - Action: Click "Copy Markdown" after successful extraction
   - Expected: Markdown is copied to clipboard

6. **Save to File Test**
   - Action: Click "Save to File" after successful extraction
   - Expected: File dialog opens, file is saved with chosen name

## Style Extraction Validation

The style extractor should capture:

✅ **Colors**
- Hex colors (#RGB, #RRGGBB, #RRGGBBAA)
- RGB/RGBA colors
- HSL/HSLA colors
- From inline styles
- From style tags

✅ **Typography**
- Font families
- Font sizes (px, em, rem, %)
- Font weights
- Line heights

✅ **Spacing**
- Margins (all units)
- Paddings (all units)
- Gaps (flexbox/grid)

✅ **Components**
- Buttons
- Links
- Headings
- Paragraphs
- Inputs
- Forms
- Navigation
- Headers
- Footers

## Frontend Testing

The frontend can be tested independently:

```bash
npm run dev
```

Then manually verify:
- URL input validation
- Button states (enabled/disabled)
- Loading indicators
- Error messages
- Success messages
- Output display
- Copy functionality (browser clipboard API)
- Save functionality (Tauri file dialog)

## Example Test HTML

For testing the extraction logic, you can use this sample HTML:

```html
<!DOCTYPE html>
<html>
<head>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            background-color: #f0f0f0;
            color: #333;
        }
        .button { 
            background: linear-gradient(to right, #667eea, #764ba2);
            padding: 10px 20px;
            border-radius: 5px;
            color: white;
        }
    </style>
</head>
<body>
    <h1 style="font-size: 2rem; margin: 20px;">Test Page</h1>
    <button class="button">Click Me</button>
</body>
</html>
```

Expected extraction should include:
- Background color: `#f0f0f0`
- Text color: `#333`
- Font family: `Arial, sans-serif`
- Font size: `2rem`
- Padding: `10px 20px`, `10px 20px`
- Margin: `20px`
- Border radius: `5px`
- Gradient: `linear-gradient(to right, #667eea, #764ba2)`
