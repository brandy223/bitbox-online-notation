"use client";
import React, {useState} from "react";
import {useRouter} from "next/navigation";
import {LoginUserPostModel} from "@/app/api/models/login-user-post-model";

const LoginPage: React.FC = () => {
    const [formData, setFormData] = useState<LoginUserPostModel>({
        login: "",
        password: "",
    });
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const router = useRouter();
    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData((prevData) => ({ ...prevData, [name]: value }));
    };
    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setIsLoading(true);
        try {
            const response = await fetch("http://localhost:8080/api/auth/login", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(formData),
                credentials: "include",
            });

            if (response.ok) {
                let mfa_code_id = await response.json();
                router.push(`/login/validate/${mfa_code_id}`);
            } else {
                // Handle different error status codes
                if (response.status === 401) {
                    setError("Invalid username or password. Please try again.");
                } else {
                    setError("An error occurred. Please try again later.");
                }
            }
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="flex font-poppins items-center justify-center">
            <div className="h-screen w-screen flex justify-center items-center dark:bg-gray-900">
                <div className="grid gap-8">
                    <div
                        id="back-div"
                        className="bg-gradient-to-r from-blue-500 to-purple-500 rounded-[26px] m-4"
                    >
                        <div className="border-[20px] border-transparent rounded-[20px] dark:bg-gray-900 bg-white shadow-lg xl:p-10 2xl:p-10 lg:p-10 md:p-10 sm:p-2 m-2">
                            <h1 className="pt-8 pb-6 font-bold dark:text-gray-400 text-5xl text-center cursor-default">
                                Log in
                            </h1>
                            <form onSubmit={handleSubmit} className="space-y-4">
                                <div>
                                    <label
                                        htmlFor="email"
                                        className="mb-2  dark:text-gray-400 text-lg"
                                    >
                                        Login
                                    </label>
                                    <input
                                        id="login"
                                        className="border p-3 dark:bg-indigo-700 dark:text-gray-300  dark:border-gray-700 shadow-md placeholder:text-base focus:scale-105 ease-in-out duration-300 border-gray-300 rounded-lg w-full"
                                        type="login"
                                        name="login"
                                        placeholder="Email or username"
                                        value={formData.login}
                                        onChange={handleInputChange}
                                        required
                                    />
                                </div>
                                <div>
                                    <label
                                        htmlFor="password"
                                        className="mb-2 dark:text-gray-400 text-lg"
                                    >
                                        Password
                                    </label>
                                    <input
                                        id="password"
                                        name="password"
                                        className="border p-3 shadow-md dark:bg-indigo-700 dark:text-gray-300  dark:border-gray-700 placeholder:text-base focus:scale-105 ease-in-out duration-300 border-gray-300 rounded-lg w-full"
                                        type="password"
                                        placeholder="Password"
                                        value={formData.password}
                                        onChange={handleInputChange}
                                        required
                                    />
                                </div>
                                <a
                                    className="group text-blue-400 transition-all duration-100 ease-in-out"
                                    href="#"
                                >
                  <span className="bg-left-bottom bg-gradient-to-r text-sm from-blue-400 to-blue-400 bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">
                    Forget your password ?
                  </span>
                                </a>
                                {error && <p style={{ color: "red" }}>{error}</p>}
                                <button
                                    className="bg-gradient-to-r dark:text-gray-300 from-blue-500 to-purple-500 shadow-lg mt-6 p-2 text-white rounded-lg w-full hover:scale-105 hover:from-purple-500 hover:to-blue-500 transition duration-300 ease-in-out"
                                    type="submit"
                                    disabled={isLoading}
                                >
                                    {isLoading ? "Logging in..." : "LOG IN"}
                                </button>
                            </form>
                            <div className="flex flex-col mt-4 items-center justify-center text-sm">
                                <h3 className="dark:text-gray-300">
                                    Don{`'`}t have an account ?
                                    <a
                                        className="group text-blue-400 transition-all duration-100 ease-in-out"
                                        href="/register"
                                    >
                    <span className="ml-2 bg-left-bottom bg-gradient-to-r from-blue-400 to-blue-400 bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">
                      Sign Up
                    </span>
                                    </a>
                                </h3>
                            </div>

                            <div className="text-gray-500 flex text-center flex-col mt-4 items-center text-sm">
                                <p className="cursor-default">
                                    By signing in, you agree to our
                                    <a
                                        className="group text-blue-400 transition-all duration-100 ease-in-out"
                                        href="#"
                                    >
                    <span className="mx-2 cursor-pointer bg-left-bottom bg-gradient-to-r from-blue-400 to-blue-400 bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">
                      Terms
                    </span>
                                    </a>
                                    and
                                    <a
                                        className="group text-blue-400 transition-all duration-100 ease-in-out"
                                        href="#"
                                    >
                    <span className="ml-2 cursor-pointer bg-left-bottom bg-gradient-to-r from-blue-400 to-blue-400 bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out">
                      Privacy Policy
                    </span>
                                    </a>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default LoginPage;
