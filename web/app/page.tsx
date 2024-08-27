'use client';

import React, {useEffect, useState} from 'react';
import {Promotion} from "@/app/api/models/promotion";
import NavBar from "@/app/components/NavBar";
import {capitalizeFirstLetter} from "@/app/utils";
import {FaPlus} from "react-icons/fa";
import {NewPromotionPostModel} from "@/app/api/models/new-promotion-post-model";
import {useRouter} from "next/navigation";
import {getCookie} from "cookies-next";

const PromotionBox: React.FC<{ promotion: Promotion }> = ({ promotion }) => {
    const router = useRouter();

    return (
        <button
            onClick={() => router.push(`/promotions/${promotion.id}`)}
            className="transition transform hover:scale-105 hover:shadow-2xl focus:outline-none"
        >
            <div className="card bg-gradient-to-r from-blue-800 to-blue-600 w-56 shadow-xl rounded-lg overflow-hidden">
                <div className="card-body text-center text-white p-4">
                    <h2 className="card-title text-lg font-bold">{capitalizeFirstLetter(promotion.title)}</h2>
                    <p className="text-sm mt-2">{`${promotion.start_year.split("-")[0]} - ${promotion.end_year.split("-")[0]}`}</p>
                </div>
            </div>
        </button>
    );
};

interface NewPromotionModalProps {
    promotions: Promotion[];
    setPromotions: React.Dispatch<React.SetStateAction<Promotion[]>>;
}

const NewPromotionModal: React.FC<NewPromotionModalProps> = ({ promotions, setPromotions }) => {
    const [formData, setFormData] = useState<NewPromotionPostModel>({
        end_year: '',
        start_year: '',
        title: '',
    });

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData((prevData) => ({ ...prevData, [name]: value }));
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            const response = await fetch("http://localhost:8080/api/promotions/", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(formData),
                credentials: "include",
            });

            // Get user id from token in cookies
            const token = getCookie("token");
            if (token === undefined) {
                setError("An error occurred. Please try again later.");
                return;
            }
            const decodedToken = JSON.parse(atob(token.split('.')[1]));
            const userId = decodedToken.sub;

            if (response.status === 201) {
                const id: string = await response.json();
                const newPromotion: Promotion = {
                    end_year: formData.end_year,
                    id,
                    start_year: formData.start_year,
                    teacher_id: userId,
                    title: formData.title,
                }
                setPromotions([...promotions, newPromotion]);
                hideModal();
            } else {
                console.log(response);
                setError("An error occurred. Please try again later.");
            }
        } catch (err) {
            console.log(err)
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <dialog id="new_promotion_modal" className="modal">
            <div className="modal-box bg-white rounded-lg shadow-lg p-6">
                <h3 className="font-bold text-xl text-gray-800">Add a Promotion</h3>
                <div className="divider"></div>
                {loading ? (
                    <p className="text-gray-500">Loading...</p>
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : null}
                <form onSubmit={handleSubmit} className="flex flex-col space-y-4">
                    <input
                        type="text"
                        placeholder="Promotion name"
                        name="title"
                        id="title"
                        value={formData.title}
                        required
                        maxLength={255}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs rounded-lg"
                    />
                    <div className="flex space-x-4">
                        <input
                            type="date"
                            placeholder="Start year"
                            name="start_year"
                            id="start_year"
                            value={formData.start_year}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-full rounded-lg"
                        />
                        <input
                            type="date"
                            placeholder="End year"
                            name="end_year"
                            id="end_year"
                            value={formData.end_year}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-full rounded-lg"
                        />
                    </div>
                    <div className="mt-6 flex justify-end">
                        <button
                            type="submit"
                            className="bg-blue-600 text-white rounded-full hover:bg-blue-700 px-6 py-2 font-bold transition"
                        >
                            Submit
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button className="btn btn-secondary">Close</button>
            </form>
        </dialog>
    );
};

const showModal = () => {
    // @ts-ignore
    document.getElementById("new_promotion_modal").showModal();
};

const hideModal = () => {
    // @ts-ignore
    document.getElementById("new_promotion_modal").close();
};

const PromotionsClientPage: React.FC = () => {
    const [promotions, setPromotions] = useState<Promotion[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchPromotions = async () => {
            try {
                const response = await fetch('http://localhost:8080/api/promotions/', {
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    credentials: "include"
                });
                const data = await response.json();
                setPromotions(data);
            } catch (err) {
                setError('Error fetching promotions');
            } finally {
                setLoading(false);
            }
        };

        fetchPromotions();
    }, []);

    return (
        <div className="min-h-screen bg-gray-100">
            <NavBar />
            <main className="container mx-auto p-6">
                <h1 className="text-3xl font-bold text-gray-800 mb-6">Promotions</h1>
                {loading ? (
                    <p className="text-gray-600">Loading...</p>
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : (
                    <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                        {promotions.length > 0 ? (
                            promotions.map((promotion) => (
                                <PromotionBox key={promotion.id} promotion={promotion} />
                            ))
                        ) : (
                            <p className="text-gray-600">No promotions found.</p>
                        )}
                    </div>
                )}
            </main>
            <NewPromotionModal promotions={promotions} setPromotions={setPromotions} />
            <button
                onClick={showModal}
                className="fixed bottom-6 right-6 bg-blue-600 text-white rounded-full p-4 shadow-lg hover:bg-blue-700 transition"
            >
                <FaPlus className="text-2xl" />
            </button>
        </div>
    );
};

export default PromotionsClientPage;
