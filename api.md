# DeGiro Connector API Documentation

## Table of Contents

- [Trading API](#trading-api)
  - [Account Info](#account-info)
  - [Account Overview](#account-overview)
  - [Account Report](#account-report)
  - [Agenda](#agenda)
  - [Client Details](#client-details)
  - [Company Profile & Ratios](#company-profile--ratios)
  - [Estimates Summaries](#estimates-summaries)
  - [Favorite Products](#favorite-products)
  - [Financial Statements](#financial-statements)
  - [Latest News](#latest-news)
  - [Notes (Product Notes)](#notes-product-notes)
  - [Order Management](#order-management)
  - [Position Report](#position-report)
  - [Product Search & Info](#product-search--info)
  - [Transactions History](#transactions-history)
  - [Upcoming Payments](#upcoming-payments)
  - [Update](#update)
- [Quotecast API](#quotecast-api)
  - [Chart Data](#chart-data)
  - [Ticker Data](#ticker-data)
  - [Ticker to Metric List](#ticker-to-metric-list)
- [Notes](#notes)

---

## Trading API

### Account Info

**Endpoint:** `GET https://trader.degiro.nl/trading/secure/v5/account/info`  
**Description:** Retrieve account information (personal, account numbers, etc).  
**Parameters:**
- `intAccount` (query)
- `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/trading/secure/v5/account/info' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID' \
-H 'Cookie: JSESSIONID=SESSION_ID'
```

---

### Account Overview

**Endpoint:** `GET https://trader.degiro.nl/portfolio-reports/secure/v6/accountoverview`  
**Description:** Retrieve account overview (portfolio, cash, etc) for a date range.  
**Parameters:**
- `fromDate`, `toDate` (query, format: YYYY-MM-DD)
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/portfolio-reports/secure/v6/accountoverview' \
--data-urlencode 'fromDate=2024-01-01' \
--data-urlencode 'toDate=2024-06-01' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID' \
-H 'Cookie: JSESSIONID=SESSION_ID'
```

---

### Account Report

**Endpoint:** `GET https://trader.degiro.nl/portfolio-reports/secure/v3/cashAccountReport`  
**Description:** Retrieve cash account report for a date range.  
**Parameters:**
- `fromDate`, `toDate`, `format` (query)
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/portfolio-reports/secure/v3/cashAccountReport' \
--data-urlencode 'fromDate=2024-01-01' \
--data-urlencode 'toDate=2024-06-01' \
--data-urlencode 'format=csv' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID' \
-H 'Cookie: JSESSIONID=SESSION_ID'
```

---

### Agenda

**Endpoint:** `GET https://trader.degiro.nl/dgtbxdsservice/secure/agenda/v2`  
**Description:** Retrieve agenda items (e.g., dividends, meetings).  
**Parameters:**
- `fromDate`, `toDate` (query)
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/dgtbxdsservice/secure/agenda/v2' \
--data-urlencode 'fromDate=2024-01-01' \
--data-urlencode 'toDate=2024-06-01' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Client Details

**Endpoint:** `GET https://trader.degiro.nl/pa/secure/client`  
**Description:** Retrieve client details (name, address, etc).  
**Parameters:**
- `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/pa/secure/client' \
--data-urlencode 'sessionId=SESSION_ID' \
-H 'Cookie: JSESSIONID=SESSION_ID'
```

---

### Company Profile & Ratios

**Endpoints:**
- Profile: `GET https://trader.degiro.nl/dgtbxdsservice/secure/company-profile/v2`
- Ratios: `GET https://trader.degiro.nl/dgtbxdsservice/secure/company-ratios`  
**Description:** Retrieve company profile and ratios by product ISIN.  
**Parameters:**
- `productIsin`, `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/dgtbxdsservice/secure/company-profile/v2' \
--data-urlencode 'productIsin=US0378331005' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Estimates Summaries

**Endpoint:** `GET https://trader.degiro.nl/dgtbxdsservice/secure/estimates-summaries`  
**Description:** Retrieve analyst estimates for a product.  
**Parameters:**
- `productIsin`, `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/dgtbxdsservice/secure/estimates-summaries' \
--data-urlencode 'productIsin=US0378331005' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Favorite Products

**Endpoint:** `GET https://trader.degiro.nl/favorites/secure/v1`  
**Description:** Retrieve favorite products.  
**Parameters:**
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/favorites/secure/v1' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Financial Statements

**Endpoint:** `GET https://trader.degiro.nl/dgtbxdsservice/secure/financial-statements/{product_isin}`  
**Description:** Retrieve financial statements for a product.  
**Parameters:**
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/dgtbxdsservice/secure/financial-statements/US0378331005' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Latest News

**Endpoint:** `GET https://trader.degiro.nl/dgtbxdsservice/secure/newsfeed/v2/latest-news`  
**Description:** Retrieve latest news.  
**Parameters:**
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/dgtbxdsservice/secure/newsfeed/v2/latest-news' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Notes (Product Notes)

**Endpoint:** `GET https://trader.degiro.nl/product-notes-service/secure/notes`  
**Description:** Retrieve notes attached to products.  
**Parameters:**
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/product-notes-service/secure/notes' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Order Management

**Endpoints:**
- Check Order: `POST https://trader.degiro.nl/trading/secure/v5/checkOrder`
- Confirm Order: `POST https://trader.degiro.nl/trading/secure/v5/order`
- Delete Order: `DELETE https://trader.degiro.nl/trading/secure/v5/order`
- Update Order: `PUT https://trader.degiro.nl/trading/secure/v5/order`
- Orders History: `GET https://trader.degiro.nl/portfolio-reports/secure/v4/order-history`  
**Description:** Place, check, update, or delete orders. Retrieve order history.  
**Parameters:**
- `intAccount`, `sessionId` (query)
- Order details in JSON body for POST/PUT

**Example (Check Order):**
```bash
curl -X POST 'https://trader.degiro.nl/trading/secure/v5/checkOrder' \
-H 'Content-Type: application/json' \
-H 'Cookie: JSESSIONID=SESSION_ID' \
-d '{"buySell":"BUY","orderType":"LIMIT","productId":360148977,"price":100,"size":1,"intAccount":1234567,"sessionId":"SESSION_ID"}'
```

---

### Position Report

**Endpoint:** `GET https://trader.degiro.nl/portfolio-reports/secure/v3/positionReport/{format}`  
**Description:** Retrieve position report in a specific format (e.g., csv, pdf).  
**Parameters:**
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/portfolio-reports/secure/v3/positionReport/csv' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Product Search & Info

**Endpoints:**
- Product Info: `GET https://trader.degiro.nl/product_search/secure/v5/products/info`
- Product Search:
  - Bonds, ETFs, Funds, Futures, Leverageds, Options, Stocks, Warrants  
**Description:** Search for products or retrieve product info.  
**Parameters:**
- Product-specific search parameters
- `intAccount`, `sessionId` (query)

**Example (Stocks):**
```bash
curl -G 'https://trader.degiro.nl/product_search/secure/v5/stocks' \
--data-urlencode 'searchText=Apple' \
--data-urlencode 'limit=10' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Transactions History

**Endpoint:** `GET https://trader.degiro.nl/portfolio-reports/secure/v4/transactions`  
**Description:** Retrieve transaction history for a date range.  
**Parameters:**
- `fromDate`, `toDate` (query)
- `intAccount`, `sessionId` (query)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/portfolio-reports/secure/v4/transactions' \
--data-urlencode 'fromDate=2024-01-01' \
--data-urlencode 'toDate=2024-06-01' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Upcoming Payments

**Endpoint:**  
`GET https://trader.degiro.nl/portfolio-reports/secure/v3/ca/{intAccount}?intAccount={intAccount};jsessionid={sessionId}`  
**Description:** Retrieve upcoming payments (e.g., dividends).  
**Parameters:**
- `intAccount`, `sessionId` (query and path)

**Example:**
```bash
curl -G 'https://trader.degiro.nl/portfolio-reports/secure/v3/ca/1234567' \
--data-urlencode 'intAccount=1234567' \
--data-urlencode 'sessionId=SESSION_ID'
```

---

### Update

**Endpoint:** `POST https://trader.degiro.nl/trading/secure/v5/update`  
**Description:** Retrieve updates for various account sections (portfolio, orders, etc).  
**Parameters:**
- `intAccount`, `sessionId` (query)
- JSON body with update requests

**Example:**
```bash
curl -X POST 'https://trader.degiro.nl/trading/secure/v5/update' \
-H 'Content-Type: application/json' \
-H 'Cookie: JSESSIONID=SESSION_ID' \
-d '[{"option":"PORTFOLIO","lastUpdated":0},{"option":"ORDERS","lastUpdated":0}]'
```

---

## Quotecast API

### Chart Data

**Endpoint:** `GET https://charting.vwdservices.com/hchart/v1/deGiro/data.js?`  
**Description:** Retrieve chart data for a product.  
**Parameters:** Various, e.g., `issueid`, `period`, `resolution`, etc.

**Example:**
```bash
curl -G 'https://charting.vwdservices.com/hchart/v1/deGiro/data.js' \
--data-urlencode 'issueid=360148977' \
--data-urlencode 'period=P1D' \
--data-urlencode 'resolution=PT1H' \
--data-urlencode 'series=price:issueid:360148977'
```

---

### Ticker Data

**Endpoint:** `POST https://degiro.quotecast.vwdservices.com/CORS/{session_id}`  
**Description:** Retrieve real-time ticker data (long-polling).  
**Parameters:** JSON body with `controlData`

**Example:**
```bash
curl -X POST 'https://degiro.quotecast.vwdservices.com/CORS/SESSION_ID' \
-H 'Content-Type: application/json' \
-d '{"controlData":"a_req(360148977.LastPrice);"}'
```

---

### Ticker to Metric List

**Description:** The response from the ticker endpoint contains:
- `DATA`: Financial data (e.g., LastPrice, LastVolume)
- `HEARTBEAT`: Sent every 5 seconds if no data updates
- `MATCHING`: Mapping of references to product IDs and labels

---

## Notes

All endpoints require authentication via `sessionId` (and often `intAccount`).  
For most endpoints, you must include the JSESSIONID cookie:
```bash
-H 'Cookie: JSESSIONID=SESSION_ID'
```

For POST/PUT/DELETE requests, use the appropriate HTTP method and JSON body.  
For product search, see request models in `trading/models/product_search.py` for all available parameters.

---
