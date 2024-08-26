"use client";
import React, {useState} from "react";
import {useParams, useRouter} from "next/navigation";

const MFACodeValidationPage: React.FC = () => {
    const {id: mfa_code_id} = useParams<{ id: string }>();

    const [formData, setFormData] = useState({
        code: "",
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
            console.log(mfa_code_id);
            const response = await fetch(`http://localhost:8080/api/auth/login/code/${mfa_code_id}`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(formData),
                credentials: "include",
            });

            if (response.ok) {
                router.push("/");
            } else {
                // Handle different error status codes
                if (response.status === 401) {
                    setError("Invalid or expired code. Please try again.");
                } else {
                    console.log(response);
                    setError("An error occurred. Please try again later.");
                }
            }
        } catch (err) {
            console.log(err);
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
                                Validate your MFA code
                            </h1>
                            <form onSubmit={handleSubmit} className="space-y-4">
                                <div>
                                    <label
                                        htmlFor="email"
                                        className="mb-2  dark:text-gray-400 text-lg"
                                    >
                                        Authentication Code
                                    </label>
                                    <input
                                        id="code"
                                        className="border p-3 dark:bg-indigo-700 dark:text-gray-300  dark:border-gray-700 shadow-md placeholder:text-base focus:scale-105 ease-in-out duration-300 border-gray-300 rounded-lg w-full"
                                        type="text"
                                        name="code"
                                        placeholder="MFA Code"
                                        value={formData.code}
                                        onChange={handleInputChange}
                                        required
                                    />
                                </div>
                                <a
                                    className="group text-blue-400 transition-all duration-100 ease-in-out"
                                    href="#"
                                >
                                </a>
                                {error && <p style={{ color: "red" }}>{error}</p>}
                                <button
                                    className="bg-gradient-to-r dark:text-gray-300 from-blue-500 to-purple-500 shadow-lg mt-6 p-2 text-white rounded-lg w-full hover:scale-105 hover:from-purple-500 hover:to-blue-500 transition duration-300 ease-in-out"
                                    type="submit"
                                    disabled={isLoading}
                                >
                                    {isLoading ? "Checking code..." : "VALIDATE"}
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default MFACodeValidationPage;
