'use client';

import React, {useEffect, useState} from 'react';
import {Promotion} from "@/app/api/models/promotion";
import NavBar from "@/app/components/NavBar";
import {capitalizeFirstLetter} from "@/app/utils";
import {FaPlus} from "react-icons/fa";
import {NewPromotionPostModel} from "@/app/api/models/new-promotion-post-model";
import {useRouter} from "next/navigation";

const PromotionBox: React.FC<{ promotion: Promotion }> = ({ promotion }) => {
    const router  = useRouter();

    return (
        <button onClick={() => router.push(`/promotions/${promotion.id}`)}>
            <div className="card bg-blue-900 w-48 shadow-xl">
                <div className="card-body items-center">
                    <h2 className="card-title">{capitalizeFirstLetter(promotion.title)}</h2>
                    <p>{`${promotion.start_year.split("-")[0]} - ${promotion.end_year.split("-")[0]}`}</p>
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

            if (response.status === 201) {
                const id: string = await response.json();
                const newPromotion: Promotion = {
                    end_year: formData.end_year,
                    id,
                    start_year: formData.start_year,
                    teacher_id: promotions[1].teacher_id,
                    title: formData.title
                }
                setPromotions([...promotions, newPromotion]);
                hideModal()
            }  else {
                setError("An error occurred. Please try again later.");
            }
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <dialog id="new_promotion_modal" className="modal">
            <div className="modal-box">
                <h3 className="font-bold text-lg">Add a promotion</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p style={{color: "red"}}>{error}</p>
                ): null}
                <form onSubmit={handleSubmit} className={"flex flex-col"}>
                    <input
                        type="text"
                        placeholder="Promotion name"
                        name="title"
                        id="title"
                        value={formData.title}
                        required
                        maxLength={255}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <div className="my-5 flex-row ">
                        <input
                            type="date"
                            placeholder="Start year"
                            name="start_year"
                            id="start_year"
                            value={formData.start_year}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-1/4"
                        />
                        <input
                            type="date"
                            placeholder="End year"
                            name="end_year"
                            id="end_year"
                            value={formData.end_year}
                            required
                            onChange={handleInputChange}
                            className="input input-bordered w-1/4"
                        />
                    </div>
                    <div className="my-4 flex justify-end w-full">
                        <button type={"submit"}
                                className={"flex- rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Submit
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    )
}

const showModal = () => {
    // @ts-ignore
    document.getElementById("new_promotion_modal").showModal();
}

const hideModal = () => {
    // @ts-ignore
    document.getElementById("new_promotion_modal").close();
}


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
                    }, credentials: "include"});
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
        <div>
            <NavBar></NavBar>
            <main>
                <h1>Promotions</h1>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p>{error}</p>
                ) : (
                    <div>
                        {promotions.length > 0 ? (
                            promotions.map((promotion) => (
                                <PromotionBox key={promotion.id} promotion={promotion}/>
                            ))
                        ) : (
                            <p>No promotions found.</p>
                        )}
                    </div>
                )}
            </main>
            <NewPromotionModal promotions={promotions} setPromotions={setPromotions} />
            <button onClick={() => showModal()}>
                <FaPlus className="size-10"/>
            </button>
        </div>
    );
};

export default PromotionsClientPage;