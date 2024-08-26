'use client';

import React from "react";
import {useRouter} from 'next/navigation'
import {FaGear, FaHouse, FaRightFromBracket} from "react-icons/fa6";

const Navbar: React.FC = () => {
    const router = useRouter();

    return (
        <div className="navbar bg-base-200">
            <div className="navbar-start">
                <button className="btn btn-ghost btn-circle ml-3" onClick={() => router.push("/")}>
                    <FaHouse className={"size-3/4"}/>
                </button>
            </div>
            <div className="navbar-center">
                <a className="btn btn-ghost text-xl">Bitbox</a>
            </div>
            <div className="navbar-end">
                <button className="btn btn-ghost btn-circle" onClick={() => router.push("/parameters")}>
                    <FaGear className={"size-1/2"}/>
                </button>
                <button className="btn btn-ghost btn-circle" onClick={() => {
                    router.push("/login")
                }}>
                    <FaRightFromBracket className={"size-1/2"}/>
                </button>
            </div>
        </div>
    );
};

export default Navbar;