gen-api-client:
	make -C backend gen-openapi
	pnpx openapi-typescript ./backend/openapi.json --o ./frontend/src/api/openapi-client.d.ts