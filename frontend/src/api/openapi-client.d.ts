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
  "/auth/applications/{slug}/login": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** Login to authenticate and generate a new JWT */
    post: operations["login"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/auth/applications/{slug}/signup": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** Signup to create a new user */
    post: operations["signup"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/auth/applications/{slug}/verify/email": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** Verify email */
    post: operations["verify_email"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/auth/applications/{slug}/verify/email/resend": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** Resend email verification code */
    post: operations["resend_verification_code"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/auth/applications/{slug}/verify/otp": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    get?: never;
    put?: never;
    /** Verify a one-time password (OTP) for two-factor authentication */
    post: operations["verify_otp"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/health": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** Verifies if the backend is healthy and it's related services */
    get: operations["health"];
    put?: never;
    post?: never;
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/providers": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** Get all providers */
    get: operations["get_providers"];
    put?: never;
    /** Create a new provider */
    post: operations["create_provider"];
    delete?: never;
    options?: never;
    head?: never;
    patch?: never;
    trace?: never;
  };
  "/providers/{id}": {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    /** Get prodiver by database ID */
    get: operations["get_provider"];
    /** Update a provider */
    put: operations["update_provider"];
    post?: never;
    /** Delete a provider */
    delete: operations["delete_provider"];
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
    post?: never;
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
      createdAt: string;
      /** Format: date-time */
      updatedAt: string;
      /** Format: int64 */
      creatorId: number;
      slug: string;
      name: string;
      description?: string | null;
      website?: string | null;
      icon?: string | null;
      passwordAuth: boolean;
      /** Format: int64 */
      passwordMinLength: number;
      /** Format: int64 */
      passwordMaxLength?: number | null;
      passwordRequiresLowercase: boolean;
      passwordRequiresUppercase: boolean;
      passwordRequiresNumber: boolean;
      passwordRequiresSpecial: boolean;
      passwordRequiresUnique: boolean;
      passwordRequiresNonCommon: boolean;
      verificationRequired: boolean;
      verificationMethod?: string | null;
      verificationCode?: string | null;
    };
    /** @description Data needed to login a user */
    LoginUserRequest: {
      /** @description The email of the user to authenticate */
      email: string;
      /** @description The password of the user to authenticate */
      password: string;
    };
    /** @description Response from login request */
    LoginUserResponse: {
      /** @description The JWT token created from login request that can be used to authenticate yourself */
      jwtToken: string;
    };
    ModifyApplication: {
      name: string;
      description?: string | null;
      website?: string | null;
      icon?: string | null;
    };
    ModifyProvider: {
      name: string;
      kind: string;
      client_id: string;
      client_secret: string;
      redirect_uri: string;
    };
    ModifyRole: {
      name: string;
      description?: string | null;
    };
    /** @description Fields to modify a user */
    ModifyUser: {
      /** @description Email of the user */
      email: string;
      /** @description Full name of the user */
      name?: string | null;
      /** @description Username of the user */
      username?: string | null;
      /** @description Picture of the user */
      picture?: string | null;
      /** @description Whether the user is enabled or not, if they are able to login/access the platform */
      disabled?: boolean;
    };
    Provider: {
      /** Format: int64 */
      id: number;
      /** Format: date-time */
      createdAt: string;
      /** Format: date-time */
      updatedAt: string;
      name: string;
      kind: string;
      clientId: string;
      clientSecret: string;
      redirectUri: string;
    };
    Role: {
      /** Format: int64 */
      id: number;
      /** Format: date-time */
      createdAt: string;
      /** Format: date-time */
      updatedAt: string;
      /** Format: int64 */
      creatorId: number;
      /** Format: int64 */
      applicationId: number;
      name: string;
      description?: string | null;
    };
    /** @description Data needed to signup/create a new user */
    SignupUserRequest: {
      /** @description The name of the user to authenticate */
      name?: string | null;
      /** @description The username of the user to authenticate */
      username?: string | null;
      /** @description The email of the user to authenticate */
      email: string;
      /** @description The password of the user to authenticate */
      password: string;
    };
    SuccessResponse: {
      success: boolean;
      message?: string | null;
    };
    /** @description A user of the platform */
    User: {
      /**
       * Format: int64
       * @description Database ID of the user
       */
      id: number;
      /**
       * Format: date-time
       * @description Time when the user was created
       */
      createdAt: string;
      /**
       * Format: date-time
       * @description Time when the user was updated
       */
      updatedAt: string;
      /** @description Email of the user */
      email: string;
      /** @description Full name of the user */
      name?: string | null;
      /** @description Username of the user */
      username?: string | null;
      /** @description Picture of the user */
      picture?: string | null;
      /** @description Whether the user is enabled or not, if they are able to login/access the platform */
      disabled: boolean;
      /** @description Language and general location (locale) of the user */
      preferredLocale?: string | null;
      /** @description Whether the user has verified their email address */
      emailVerified: boolean;
      /** @description Whether two-factor authentication is enabled for this user */
      twoFactorEnabled: boolean;
      /**
       * Format: date-time
       * @description Time when the verification code was created
       */
      verificationCodeCreatedAt?: string | null;
    };
    VerifyEmailResponse: {
      success: boolean;
    };
    VerifyOtpResponse: {
      success: boolean;
      redirect?: string | null;
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
  login: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        /** @description Application slug */
        slug: string;
      };
      cookie?: never;
    };
    /** @description User information needed to authenticate a user */
    requestBody: {
      content: {
        "application/json": components["schemas"]["LoginUserRequest"];
      };
    };
    responses: {
      /** @description User logged in successfully */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["LoginUserResponse"];
        };
      };
    };
  };
  signup: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        /** @description Application slug */
        slug: string;
      };
      cookie?: never;
    };
    /** @description User information needed to create new user */
    requestBody: {
      content: {
        "application/json": components["schemas"]["SignupUserRequest"];
      };
    };
    responses: {
      /** @description User created successfully */
      201: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  verify_email: {
    parameters: {
      query: {
        /** @description Verification token */
        token: string;
      };
      header?: never;
      path: {
        /** @description Application slug */
        slug: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["VerifyEmailResponse"];
        };
      };
    };
  };
  resend_verification_code: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        /** @description Application slug */
        slug: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["SuccessResponse"];
        };
      };
      /** @description User already verified */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Too many requests - must wait before requesting new code */
      429: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Email service not configured */
      500: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  verify_otp: {
    parameters: {
      query: {
        /** @description One-time password to verify */
        otp: string;
      };
      header?: never;
      path: {
        /** @description Application slug */
        slug: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description OTP verification successful */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["VerifyOtpResponse"];
        };
      };
      /** @description Invalid OTP provided */
      400: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description 2FA is not enabled for this user */
      403: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description TOTP secret not found for user */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  health: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Backend is healthy */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  get_providers: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description List matching providers by query */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Provider"][];
        };
      };
    };
  };
  create_provider: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/json": components["schemas"]["ModifyProvider"];
      };
    };
    responses: {
      /** @description Provider created successfully */
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
  get_provider: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        /** @description Provider database ID */
        id: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Get provider by ID */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "application/json": components["schemas"]["Provider"];
        };
      };
      /** @description Provider was not found */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  update_provider: {
    parameters: {
      query?: never;
      header?: never;
      path?: never;
      cookie?: never;
    };
    requestBody: {
      content: {
        "application/json": components["schemas"]["ModifyProvider"];
      };
    };
    responses: {
      /** @description Provider updated successfully */
      200: {
        headers: {
          [name: string]: unknown;
        };
        content: {
          "text/plain": string;
        };
      };
      /** @description Provider was not updated */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
    };
  };
  delete_provider: {
    parameters: {
      query?: never;
      header?: never;
      path: {
        /** @description Provider database ID */
        id: string;
      };
      cookie?: never;
    };
    requestBody?: never;
    responses: {
      /** @description Provider deleted successfully */
      201: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
      };
      /** @description Provider not found */
      404: {
        headers: {
          [name: string]: unknown;
        };
        content?: never;
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
