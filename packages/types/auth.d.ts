export interface LoginResponse {
  message: string;
  redirectUrl: string;
}

export interface UserProfile {
  id: string;
  username: string;
  email?: string;
  avatarUrl?: string;
}
