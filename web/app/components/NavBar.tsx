import React from "react";
import Link from "next/link";
import {useRouter} from "next/navigation";

const NavBar: React.FC = () => {
    const router = useRouter();

    const logout = async () => {
        await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/logout`, {
            method: "POST",
            credentials: "include",
        });
        router.push("/login");
    };

    return (
        <nav className="bg-blue-600 text-white p-4 shadow-md">
            <div className="container mx-auto flex justify-between items-center">
                <Link href="/" className="text-xl font-bold hover:text-blue-200">
                    Home
                </Link>
                <button
                    onClick={logout}
                    className="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                >
                    Logout
                </button>
            </div>
        </nav>
    );
};

export default NavBar;
