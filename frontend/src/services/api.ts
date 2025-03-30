const wmsApiUrl = import.meta.env.VITE_API_BASEURL + "/wms";
const loginUrl = import.meta.env.VITE_LOGIN_URL;

export const mockLogin = async (): Promise<void> => {
  const response = await fetch(wmsApiUrl, {
    method: "GET",
    headers: { Authorization: "Bearer valid_token" },
    credentials: "include",
  });

  if (!response.ok) {
    throw new Error("Login failed");
  }
};

export const fetchWmsGroups = async (): Promise<WmsGroup[]> => {
  const response = await fetch(wmsApiUrl, {
    method: "GET",
    credentials: "include",
  });
  if (response.status == 401) {
    window.location.href = loginUrl;
    throw new Error("Unauthorized. Redirecting to login.");
  }
  if (!response.ok) {
    throw new Error("Failed to fetch WMS list");
  }
  return await response.json();
};

export const fetchWmsById = async (id: number): Promise<Wms> => {
  const response = await fetch(`${wmsApiUrl}/${id}`, {
    method: "GET",
    credentials: "include",
  });
  if (!response.ok) {
    throw new Error(`Failed to fetch WMS with ID ${id}`);
  }
  return await response.json();
};
