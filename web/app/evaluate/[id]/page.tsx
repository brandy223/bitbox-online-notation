'use client'

import React, {useEffect} from "react";
import NavBar from "@/app/components/NavBar";
import {useParams, useRouter} from "next/navigation";
import {deleteCookie} from "cookies-next";

const GetEvaluationTokenPage: React.FC = () => {
    const {id: token_id} = useParams<{ id: string }>();
    const router  = useRouter();

    useEffect(() => {
        const fetchToken = async () => {
            if (!token_id) {
                return;
            }

            deleteCookie('token');

            const api_url = process.env.NEXT_PUBLIC_API_URL;
            const response = await fetch(`${api_url}/token/evaluation?id=${token_id}`, {
                headers: {'Content-Type': 'application/json',},
                credentials: "include",
            });

            if (response.status === 200) {
                router.push(`/evaluate`);
            } else {
                router.push(`/login`);
            }
        }

        fetchToken();
    }, [token_id]);

    return (
        <div>
            <NavBar/>
        </div>
    );
}

export default GetEvaluationTokenPage;