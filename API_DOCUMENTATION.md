# Rfamily API Documentation

## 🎉 Interactive API Documentation Now Available!

Your Rfamily web application now includes **Swagger/OpenAPI documentation** with an interactive "Try it out" feature!

## Quick Access

Start the server:
```bash
cargo run -p rfamily-web
```

Then visit:
- **Swagger UI**: http://localhost:3000/api/docs
- **OpenAPI Spec**: http://localhost:3000/api/openapi.json
- **Web Interface**: http://localhost:3000

## What You Get

### 📚 Interactive Documentation
- **Try it out**: Execute API calls directly from your browser
- **Request/Response examples**: See exactly what to send and expect
- **Schema definitions**: Complete type definitions for all models
- **Error codes**: Documented response codes for each endpoint

### 🎯 API Endpoints (3 Categories)

#### Presets
- `GET /api/presets` - List all 51 language presets
- `GET /api/presets/{name}` - Get specific preset details
- `GET /api/example` - Get example ruleset template

#### Validation
- `POST /api/validate` - Validate a custom ruleset

#### Generation
- `POST /api/preview` - Generate preview (10-100 records)
- `POST /api/generate` - Generate full GEDCOM file

## Features

### ✅ What's Included

**OpenAPI 3.0.3 Specification:**
- Complete request/response schemas
- Parameter validation (min/max values)
- Error response documentation
- Example values for all types

**Swagger UI Features:**
- Interactive API testing
- Request builder with validation
- Response visualization
- Model schemas with descriptions
- Organized by tags (Presets, Validation, Generation)

**Enhanced Documentation:**
- Detailed descriptions for all endpoints
- Parameter constraints (e.g., count: 10-100 for preview)
- Response status codes with explanations
- Contact information and versioning

## Using the Interactive Documentation

### 1. Explore Endpoints
Visit http://localhost:3000/api/docs and you'll see all endpoints organized by category.

### 2. Try an Endpoint
1. Click on an endpoint (e.g., `GET /api/presets`)
2. Click "Try it out"
3. Click "Execute"
4. See the response!

### 3. Test Generation
1. Click `POST /api/preview`
2. Click "Try it out"
3. Edit the request body:
   ```json
   {
     "count": 50,
     "preset_name": "japanese"
   }
   ```
4. Click "Execute"
5. See the generated GEDCOM in the response!

## Example API Calls

### List Presets
```bash
curl http://localhost:3000/api/presets
```

Response includes:
- name (e.g., "english")
- display_name (e.g., "English")
- description
- region (e.g., "Europe")

### Generate Preview
```bash
curl -X POST http://localhost:3000/api/preview \
  -H "Content-Type: application/json" \
  -d '{
    "count": 50,
    "preset_name": "spanish"
  }'
```

Response includes:
- gedcom (string) - Generated GEDCOM content
- statistics (object):
  - total_individuals
  - males
  - females
  - families
  - generation_time_ms

### Validate Ruleset
```bash
curl -X POST http://localhost:3000/api/validate \
  -H "Content-Type: application/json" \
  -d '{
    "ruleset": {
      "names": {...},
      "dates": {...}
    }
  }'
```

Response includes:
- valid (boolean)
- errors (array of strings)
- warnings (array of strings)

## Schema Definitions

All request and response types are fully documented:

### PreviewRequest
```json
{
  "count": 50,                      // 10-100
  "preset_name": "english",         // Optional
  "ruleset": {...}                  // Optional (if not using preset)
}
```

### PreviewResponse
```json
{
  "gedcom": "0 HEAD\n1 SOUR...",
  "statistics": {
    "total_individuals": 50,
    "males": 26,
    "females": 24,
    "families": 12,
    "generation_time_ms": 15
  }
}
```

### ErrorResponse
```json
{
  "error": "Count must be <= 100",
  "details": null
}
```

## Response Status Codes

### 2xx Success
- `200 OK` - Request successful
- `201 Created` - Resource created (if applicable)

### 4xx Client Errors
- `400 Bad Request` - Invalid parameters (e.g., count out of range)
- `404 Not Found` - Preset not found
- `413 Payload Too Large` - Count exceeds maximum (10M)

### 5xx Server Errors
- `500 Internal Server Error` - Generation failed

## OpenAPI Specification

Access the raw OpenAPI spec at:
```
http://localhost:3000/api/openapi.json
```

This can be imported into:
- **Postman**: Import → Link → paste URL
- **Insomnia**: Import → From URL
- **API clients**: Most support OpenAPI 3.0.3

## Developer Integration

### Generate Client Libraries

Use the OpenAPI spec to generate client libraries:

```bash
# Download spec
curl http://localhost:3000/api/openapi.json > openapi.json

# Generate TypeScript client
npx @openapitools/openapi-generator-cli generate \
  -i openapi.json \
  -g typescript-axios \
  -o ./client

# Generate Python client
openapi-generator-cli generate \
  -i openapi.json \
  -g python \
  -o ./python-client
```

### Use in Your Application

**TypeScript/JavaScript:**
```typescript
import { RfamilyApi } from './client';

const api = new RfamilyApi({ basePath: 'http://localhost:3000' });

// List presets
const presets = await api.listPresets();

// Generate preview
const preview = await api.preview({
  count: 50,
  preset_name: 'japanese'
});
```

**Python:**
```python
from rfamily_client import ApiClient, PresetsApi, GenerationApi

api_client = ApiClient(configuration)
presets_api = PresetsApi(api_client)

# List presets
presets = presets_api.list_presets()

# Generate preview
generation_api = GenerationApi(api_client)
preview = generation_api.preview(preview_request)
```

## Benefits for Developers

### 🚀 Faster Integration
- No need to read docs - try endpoints directly
- See exactly what parameters are required
- Copy working examples

### 🔍 Clear Contracts
- Type-safe request/response definitions
- Validation rules visible
- Error responses documented

### 🧪 Easy Testing
- Test endpoints without writing code
- Validate responses match schema
- Debug issues quickly

## Configuration

The API documentation is automatically configured with:
- Title: "Rfamily API"
- Version: "0.1.0"
- Description: Full API description
- Contact: GitHub project link
- License: MIT

## Updating Documentation

When you add new endpoints:

1. Add `#[utoipa::path]` annotation to handler
2. Add handler to `ApiDoc` paths list in main.rs
3. Add request/response types to components/schemas
4. Rebuild: `cargo build -p rfamily-web`
5. Restart server

Documentation updates automatically!

## Troubleshooting

**Swagger UI not loading:**
- Check server is running: `curl http://localhost:3000/api/openapi.json`
- Clear browser cache
- Check browser console for errors

**Endpoints not appearing:**
- Verify `#[utoipa::path]` annotation exists
- Check endpoint is added to ApiDoc paths
- Rebuild the project

**"Try it out" fails:**
- Check request body is valid JSON
- Verify required fields are provided
- Check server logs for errors

## Summary

You now have **professional-grade API documentation** including:
✅ Interactive Swagger UI at /api/docs
✅ OpenAPI 3.0.3 specification
✅ Complete schema definitions
✅ Request/response examples
✅ "Try it out" testing
✅ Error documentation
✅ Client library generation support

**Access it now:**
```bash
cargo run -p rfamily-web
# Open: http://localhost:3000/api/docs
```

Enjoy your fully documented REST API! 🎉
