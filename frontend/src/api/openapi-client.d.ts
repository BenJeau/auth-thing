/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
    "/applications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get all applications */
        get: operations["get_applications"];
        put?: never;
        /** Create a new application */
        post: operations["create_application"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/applications/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get application by database ID */
        get: operations["get_application"];
        /** Update an application */
        put: operations["update_application"];
        post?: never;
        /** Delete an application */
        delete: operations["delete_application"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/applications/{id}/roles": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get roles for application */
        get: operations["get_application_roles"];
        put?: never;
        /** Create a new role */
        post: operations["create_role"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/roles": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get all roles */
        get: operations["get_roles"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/roles/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get role by database ID */
        get: operations["get_role"];
        /** Update a role */
        put: operations["update_role"];
        post?: never;
        /** Delete a role */
        delete: operations["delete_role"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get all users */
        get: operations["get_users"];
        put?: never;
        /** Create a new user */
        post: operations["create_user"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get user by database ID */
        get: operations["get_user"];
        /** Update a user */
        put: operations["update_user"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}/roles": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get user roles */
        get: operations["get_user_roles"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/users/{id}/roles/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Assign a role to a user */
        post: operations["assign_user_role"];
        /** Remove a role from a user */
        delete: operations["remove_user_role"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        Application: {
            /** Format: int64 */
            id: number;
            /** Format: date-time */
            created_at: string;
            /** Format: date-time */
            updated_at: string;
            /** Format: int64 */
            creator_id: number;
            name: string;
            description?: string | null;
            website?: string | null;
            icon?: string | null;
        };
        ModifyApplication: {
            name: string;
            description?: string | null;
            website?: string | null;
            icon?: string | null;
        };
        ModifyRole: {
            name: string;
            description?: string | null;
        };
        ModifyUser: {
            email: string;
            name?: string | null;
            username?: string | null;
            picture?: string | null;
            disabled?: boolean;
            verified?: boolean;
        };
        Role: {
            /** Format: int64 */
            id: number;
            /** Format: date-time */
            created_at: string;
            /** Format: date-time */
            updated_at: string;
            /** Format: int64 */
            creator_id: number;
            /** Format: int64 */
            application_id: number;
            name: string;
            description?: string | null;
        };
        User: {
            /** Format: int64 */
            id: number;
            /** Format: date-time */
            created_at: string;
            /** Format: date-time */
            updated_at: string;
            email: string;
            name?: string | null;
            username?: string | null;
            picture?: string | null;
            disabled: boolean;
            verified: boolean;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    get_applications: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description List matching applications by query */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Application"][];
                };
            };
        };
    };
    create_application: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyApplication"];
            };
        };
        responses: {
            /** @description Application created successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
        };
    };
    get_application: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Application database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Get application by ID */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Application"];
                };
            };
            /** @description Application was not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    update_application: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyApplication"];
            };
        };
        responses: {
            /** @description Application updated successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
            /** @description Application was not updated */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    delete_application: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Application database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Application deleted successfully */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Application not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_application_roles: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Application database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description List matching roles by query */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Role"][];
                };
            };
        };
    };
    create_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Application database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyRole"];
            };
        };
        responses: {
            /** @description Role created successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
        };
    };
    get_roles: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description List matching roles by query */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Role"][];
                };
            };
        };
    };
    get_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Role database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Get role by ID */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Role"];
                };
            };
            /** @description Role was not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    update_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Role database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyRole"];
            };
        };
        responses: {
            /** @description Role updated successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
            /** @description Role was not updated */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    delete_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Role database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Role deleted successfully */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Role not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_users: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description List matching users by query */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["User"][];
                };
            };
        };
    };
    create_user: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyUser"];
            };
        };
        responses: {
            /** @description User created successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
        };
    };
    get_user: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description User database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Get user by ID */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["User"];
                };
            };
            /** @description User was not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    update_user: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModifyUser"];
            };
        };
        responses: {
            /** @description User updated successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
            /** @description User was not updated */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_user_roles: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description User database ID */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description List matching user roles by query */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Role"][];
                };
            };
        };
    };
    assign_user_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description User database ID */
                id: string;
                /** @description Role database ID */
                role_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Role assigned successfully */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
            /** @description Role doesn't exist */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    remove_user_role: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description User database ID */
                id: string;
                /** @description Role database ID */
                role_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Role removed successfully */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Role doesn't exist */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
}
