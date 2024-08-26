'use client'

import React, {useEffect, useState} from "react";
import {Promotion} from "@/app/api/models/promotion";
import NavBar from "@/app/components/NavBar";
import {Student} from "@/app/api/models/student";
import {Project} from "@/app/api/models/project";
import {capitalizeFirstLetter, showModal} from "@/app/utils";
import {useParams} from "next/navigation";
import {FaPlus} from "react-icons/fa";
import NewStudentModal from "@/app/components/modals/NewStudentModal";
import NewProjectModal from "@/app/components/modals/NewProjectModal";
import DeleteStudentModal from "@/app/components/modals/DeleteStudentModal";
import StudentComponent from "@/app/components/data/StudentComponent";
import Link from "next/link";

async function getStudentsFromPromotion(promotion_id: string): Promise<Student[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/students/promotion/${promotion_id}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as Student[];
}

async function getProjectsFromPromotion(promotion_id: string): Promise<Project[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/projects/promotion/${promotion_id}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as Project[];
}

async function getPromotion(promotion_id: string): Promise<Promotion> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/promotions/${promotion_id}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as Promotion;
}

async function updateStudent(student_id: string, student: Partial<Student>) {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/students/${student_id}`, {
        method: "PUT",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            name: student.name,
            surname: student.surname,
            email: student.email,
        }),
        credentials: "include",
    });
    return response.status;
}

const PromotionDetails: React.FC<{ promotion: Promotion }> = ({promotion}) => {
    return (
        <div className="px-4 py-2">
            <h2 className="">Promotion details</h2>
            <div className="divider"></div>
            <div>
                <h3>{capitalizeFirstLetter(promotion.title)}</h3>
                <p>Start date : {promotion.start_year}</p>
                <p>End date : {promotion.end_year}</p>
            </div>
        </div>
    )
}



const ProjectComponent: React.FC<{ project: Project }> = ({project}) => {
    return (
        <Link href={`/projects/${project.id}`}>
            <div className="px-4 py-2">
                <p>{capitalizeFirstLetter(project.name)}</p>
            </div>
        </Link>
    )
}

const PromotionDetailsComponent: React.FC<{ promotion_id: string }> = ({promotion_id}) => {
    const [promotion, setPromotion] = useState<Promotion>({
        id: '',
        title: '',
        start_year: '',
        end_year: '',
        teacher_id: '',
    });
    const [promotionLoading, setPromotionLoading] = useState(true);
    const [promotionError, setPromotionError] = useState<string | null>(null);

    useEffect(() => {
        const fetchPromotion = async () => {
            try {
                setPromotionLoading(true);
                const promotionData = await getPromotion(promotion_id);
                setPromotion(promotionData);
            } catch (err) {
                setPromotionError('Error fetching promotion');
            } finally {
                setPromotionLoading(false);
            }
        };

        fetchPromotion();
    }, [promotion_id]);

    return (
        <div>
            {promotionLoading ? (
                <p>Loading promotion...</p>
            ) : promotionError ? (
                <p>{promotionError}</p>
            ) : (
                <PromotionDetails promotion={promotion}/>
            )}
        </div>
    )
}


const PromotionPage: React.FC = () => {
    const [students, setStudents] = useState<Student[]>([]);
    const [selectedStudent, setSelectedStudent] = useState<Student | null>(null);
    const [projects, setProjects] = useState<Project[]>([]);
    const [studentsLoading, setStudentsLoading] = useState(true);
    const [projectsLoading, setProjectsLoading] = useState(true);
    const [studentsError, setStudentsError] = useState<string | null>(null);
    const [projectsError, setProjectsError] = useState<string | null>(null);

    const {id: promotion_id} = useParams<{ id: string }>();

    useEffect(() => {
        const fetchStudents = async () => {
            try {
                let students = await getStudentsFromPromotion(promotion_id);
                setStudents(students);
            } catch (err) {
                setStudentsError('Error fetching students')
            } finally {
                setStudentsLoading(false);
            }
        }
        const fetchProjects = async () => {
            try {
                let projects = await getProjectsFromPromotion(promotion_id);
                setProjects(projects);
            } catch (err) {
                setProjectsError('Error fetching projects')
            } finally {
                setProjectsLoading(false);
            }
        }
        fetchStudents();
        fetchProjects();
    }, [promotion_id])

    return (
        <div>
            <NavBar/>
            <div className="flex">
                <div className="flex-row">
                    <div className="flex-col">
                        <PromotionDetailsComponent promotion_id={promotion_id}/>
                        <div className="flex-col">
                            <h2>Students</h2>
                            <div className="divider"></div>

                            {studentsLoading ? (
                                <p>Loading students...</p>
                            ) : studentsError ? (
                                <p>{studentsError}</p>
                            ) : students && students.length > 0 ? (
                                students.map((student) => (
                                    <StudentComponent key={student.id} student={student}
                                                      onDelete={() => {
                                                            setSelectedStudent(student);
                                                          showModal("delete_student_modal")
                                                      }}
                                                        onUpdate={async () => {
                                                            const status = await updateStudent(student.id, student)
                                                            if (status === 200) {
                                                                const updatedStudents = students.map(s => s.id === student.id ? student : s);
                                                                setStudents(updatedStudents);
                                                            }
                                                        }}
                                    />
                                ))
                            ) : (
                                <p>No students found</p>
                            )}
                            <NewStudentModal students={students} setStudents={setStudents} promotion_id={promotion_id} />
                            <button onClick={() => showModal("new_student_modal")}>
                                <FaPlus className="size-10"/>
                            </button>
                            <DeleteStudentModal students={students} setStudents={setStudents} student={selectedStudent} />
                        </div>
                    </div>
                    <div className="flex-col">
                        <h2>Projects</h2>
                        <div className="divider"></div>

                        {projectsLoading ? (
                            <p>Loading projects...</p>
                        ) : projectsError ? (
                            <p>{projectsError}</p>
                        ) : projects && projects.length > 0 ? (
                            projects.map((project) => (
                                <ProjectComponent key={project.id} project={project} />
                            ))
                        ) : (
                            <p>No projects found</p>
                        )}
                        <NewProjectModal projects={projects} setProjects={setProjects} promotion_id={promotion_id} />
                        <button onClick={() => showModal("new_project_modal")}>
                            <FaPlus className="size-10"/>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default PromotionPage;